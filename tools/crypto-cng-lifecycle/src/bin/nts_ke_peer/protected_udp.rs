// SPDX-License-Identifier: GPL-2.0-only
//! One owned IPv4 loopback socket for the standalone protected peer.

use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::num::NonZeroU16;
use std::time::{Duration, Instant};

use zeroize::{Zeroize, Zeroizing};

use super::{
    Budget, ClientExit, Failure,
    protected_packet::{
        Correlation, DENY_BYTES, NAK_BYTES, PacketFailure, REPLY_BYTES, REQUEST_BYTES,
        SuccessorCorrelation, admit_and_seal, admit_and_seal_initial_deny, admit_successor,
        correlated_nak, seal_deny, seal_ordinary_successor,
    },
};

/// Windows reports an overlong UDP datagram as a receive error.
#[cfg(windows)]
const WSAEMSGSIZE: i32 = 10_040;

/// The selected reply sequence for the sole authenticated successor request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SuccessorReply {
    /// One authenticated DENY.
    Deny,
    /// One authenticated ordinary reply with a fresh replacement cookie.
    Ordinary,
    /// One NTSN immediately followed by the authenticated ordinary reply.
    NakThenOrdinary,
}

/// One kernel-assigned loopback endpoint and its original invocation deadline.
pub(super) struct ProtectedUdp {
    /// Owns the sole socket through completion or first refusal.
    socket: UdpSocket,
    /// The nonzero local port bound on the same socket used for exchange.
    port: NonZeroU16,
    /// Prevents supplying a refreshed lifetime after port publication.
    deadline: Instant,
}

impl ProtectedUdp {
    /// Bind the endpoint before its port is available to the KE producer.
    ///
    /// # Errors
    /// Refuse expiration, socket setup failure or an invalid local endpoint.
    pub(super) fn bind(budget: &Budget) -> Result<Self, Failure> {
        budget.poll(Instant::now())?;
        let socket: UdpSocket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .map_err(|error| Failure::Io(error.kind()))?;
        let local: SocketAddr = socket
            .local_addr()
            .map_err(|error| Failure::Io(error.kind()))?;
        let SocketAddr::V4(local) = local else {
            return Err(Failure::Arguments);
        };
        if *local.ip() != Ipv4Addr::LOCALHOST {
            return Err(Failure::Arguments);
        }
        let port: NonZeroU16 = NonZeroU16::new(local.port()).ok_or(Failure::Arguments)?;
        budget.poll(Instant::now())?;
        Ok(Self {
            socket,
            port,
            deadline: budget.deadline,
        })
    }

    /// The kernel-assigned port for this still-owned socket.
    pub(super) const fn port(&self) -> NonZeroU16 {
        self.port
    }

    /// Consume this socket for one request with the completed KE session's keys.
    ///
    /// Bound each receive wait by the observed time remaining. With the
    /// original invocation still current, authenticate the first datagram
    /// and try one nonblocking send to its exact source endpoint.
    /// Any terminal outcome ends this socket's lifetime, including uncertainty
    /// about an attempted send.
    ///
    /// # Errors
    /// Preserve deadline, I/O, sender, packet and nonce-generation refusals.
    pub(super) fn exchange(
        self,
        budget: &Budget,
        c2s: &[u8; 32],
        s2c: &[u8; 32],
        fill: impl FnOnce(&mut [u8; 16]) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.exchange_with_clock(budget, c2s, s2c, fill, Instant::now)
    }

    /// Reuse the real exchange with an internal clock seam for the late control.
    ///
    /// # Errors
    /// Reject an altered deadline or incomplete operation. A late observation
    /// after a successful OS send cannot undo the transmitted datagram.
    fn exchange_with_clock(
        self,
        budget: &Budget,
        c2s: &[u8; 32],
        s2c: &[u8; 32],
        fill: impl FnOnce(&mut [u8; 16]) -> Result<(), Failure>,
        now: impl Fn() -> Instant,
    ) -> Result<(), Failure> {
        if self.deadline != budget.deadline {
            return Err(Failure::Deadline);
        }

        self.initial_exchange(budget, c2s, s2c, fill, &now)
            .map(|_: Correlation| ())
    }

    /// Return one authenticated initial DENY, erase both exporters, and observe client exit.
    ///
    /// The originally advertised socket and its deadline remain owned through the
    /// receive-only observation. Any datagram after the one reply refuses completion.
    ///
    /// # Errors
    /// Preserve exchange refusal, an unexpected datagram or acknowledgement failure.
    pub(super) fn exchange_initial_deny_observe_exit(
        self,
        budget: &Budget,
        c2s: &mut Zeroizing<[u8; 32]>,
        s2c: &mut Zeroizing<[u8; 32]>,
        fill: impl FnOnce(&mut [u8; 16]) -> Result<(), Failure>,
        read_exit: impl FnMut() -> Result<Option<ClientExit>, Failure>,
    ) -> Result<(), Failure> {
        let reply: Result<(), Failure> = self.reply_to_initial_deny(budget, c2s, s2c, fill);
        c2s.zeroize();
        s2c.zeroize();
        reply?;
        self.observe_client_exit(budget, read_exit, Instant::now)
    }

    /// Use the KE cookie and original directional exporters for the only reply.
    ///
    /// # Errors
    /// Preserve the first deadline, I/O, entropy or packet refusal.
    fn reply_to_initial_deny(
        &self,
        budget: &Budget,
        c2s: &[u8; 32],
        s2c: &[u8; 32],
        fill: impl FnOnce(&mut [u8; 16]) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        if self.deadline != budget.deadline {
            return Err(Failure::Deadline);
        }
        let (request, sender): (Zeroizing<[u8; REQUEST_BYTES + 1]>, SocketAddrV4) =
            self.receive(budget, &Instant::now)?;
        let mut nonce: Zeroizing<[u8; 16]> = Zeroizing::new([0; 16]);
        budget.poll(Instant::now())?;
        fill(&mut nonce)?;
        budget.poll(Instant::now())?;
        let reply: Zeroizing<[u8; DENY_BYTES]> =
            admit_and_seal_initial_deny(&request[..REQUEST_BYTES], c2s, s2c, &nonce)
                .map_err(Failure::Datagram)?;
        self.send_reply(budget, &Instant::now, sender, &reply[..])
    }

    /// Use one listener and the original exporters for an ordinary reply and a DENY.
    ///
    /// The second receive can come from a new ephemeral port. The full request
    /// must authenticate with the issued cookie and distinct UID and transmit
    /// token. Each leg reads at most one complete datagram and attempts at most
    /// one send. A failure never opens another attempt or a renewed deadline.
    ///
    /// # Errors
    /// Preserve the first deadline, I/O, entropy or packet refusal.
    pub(super) fn exchange_successor_deny(
        self,
        budget: &Budget,
        c2s: &[u8; 32],
        s2c: &[u8; 32],
        fill: impl FnMut(&mut [u8; 16]) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.reply_to_successor(budget, c2s, s2c, fill, SuccessorReply::Deny)
    }

    /// Complete the selected two exchanges, erase the exporters, then observe the socket.
    ///
    /// All reply sends and exit observation share the original budget and endpoint.
    ///
    /// # Errors
    /// Preserve exchange refusal, an unexpected datagram or acknowledgement failure.
    pub(super) fn exchange_successor_observe_exit(
        self,
        budget: &Budget,
        c2s: &mut Zeroizing<[u8; 32]>,
        s2c: &mut Zeroizing<[u8; 32]>,
        fill: impl FnMut(&mut [u8; 16]) -> Result<(), Failure>,
        read_exit: impl FnMut() -> Result<Option<ClientExit>, Failure>,
        successor_reply: SuccessorReply,
    ) -> Result<(), Failure> {
        let replies: Result<(), Failure> =
            self.reply_to_successor(budget, c2s, s2c, fill, successor_reply);
        c2s.zeroize();
        s2c.zeroize();
        replies?;
        self.observe_client_exit(budget, read_exit, Instant::now)
    }

    /// Complete the initial and successor legs before any observation.
    ///
    /// Authenticate exactly one request per leg. Both successor responses in
    /// the NTSN sequence use the second request's checked correlation and sender.
    ///
    /// # Errors
    /// Preserve the first deadline, I/O, entropy or packet refusal.
    fn reply_to_successor(
        &self,
        budget: &Budget,
        c2s: &[u8; 32],
        s2c: &[u8; 32],
        mut fill: impl FnMut(&mut [u8; 16]) -> Result<(), Failure>,
        successor_reply: SuccessorReply,
    ) -> Result<(), Failure> {
        if self.deadline != budget.deadline {
            return Err(Failure::Deadline);
        }
        let initial: Correlation =
            self.initial_exchange(budget, c2s, s2c, &mut fill, &Instant::now)?;
        budget.poll(Instant::now())?;
        self.socket
            .set_nonblocking(false)
            .map_err(|error| Failure::Io(error.kind()))?;
        let (request, sender): (Zeroizing<[u8; REQUEST_BYTES + 1]>, SocketAddrV4) =
            self.receive(budget, &Instant::now)?;
        let successor: SuccessorCorrelation =
            admit_successor(&request[..REQUEST_BYTES], c2s, &initial).map_err(Failure::Datagram)?;
        let mut nonce: Zeroizing<[u8; 16]> = Zeroizing::new([0; 16]);
        budget.poll(Instant::now())?;
        fill(&mut nonce)?;
        budget.poll(Instant::now())?;
        match successor_reply {
            SuccessorReply::Deny => {
                let reply: Zeroizing<[u8; DENY_BYTES]> =
                    seal_deny(successor, s2c, &nonce).map_err(Failure::Datagram)?;
                self.send_reply(budget, &Instant::now, sender, &reply[..])
            }
            SuccessorReply::Ordinary | SuccessorReply::NakThenOrdinary => {
                let nak: Option<[u8; NAK_BYTES]> = (successor_reply
                    == SuccessorReply::NakThenOrdinary)
                    .then(|| correlated_nak(&successor));
                let reply: Zeroizing<[u8; REPLY_BYTES]> =
                    seal_ordinary_successor(successor, s2c, &nonce).map_err(Failure::Datagram)?;
                if let Some(nak) = nak {
                    self.send_reply(budget, &Instant::now, sender, &nak)?;
                }
                self.send_reply(budget, &Instant::now, sender, &reply[..])
            }
        }
    }

    /// Hold the existing nonblocking socket until the exact exit acknowledgement.
    ///
    /// Check the queue again after acknowledgement to cover any packet arriving
    /// between the first probe and reading the marker.
    ///
    /// # Errors
    /// Refuse late completion, extra datagrams or failed socket/marker reads.
    fn observe_client_exit(
        self,
        budget: &Budget,
        mut read_exit: impl FnMut() -> Result<Option<ClientExit>, Failure>,
        now: impl Fn() -> Instant,
    ) -> Result<(), Failure> {
        loop {
            budget.poll(now())?;
            self.refuse_extra_datagram()?;
            let exited: Option<ClientExit> = read_exit()?;
            let pause: Duration = budget.poll(now())?;
            if let Some(ClientExit) = exited {
                self.refuse_extra_datagram()?;
                budget.poll(now())?;
                return Ok(());
            }
            std::thread::sleep(pause);
        }
    }

    /// Observe at most one queued datagram without parsing or replying.
    ///
    /// # Errors
    /// Refuse any datagram, including zero-length and truncated receives, and
    /// any I/O error other than the nonblocking empty queue.
    fn refuse_extra_datagram(&self) -> Result<(), Failure> {
        let mut sentinel: [u8; 1] = [0; 1];
        match self.socket.recv_from(&mut sentinel) {
            Ok(_) => Err(Failure::UnexpectedDatagram),
            Err(error) if error.kind() == ErrorKind::WouldBlock => Ok(()),
            #[cfg(windows)]
            Err(error) if error.raw_os_error() == Some(WSAEMSGSIZE) => {
                Err(Failure::UnexpectedDatagram)
            }
            Err(error) => Err(Failure::Io(error.kind())),
        }
    }

    /// Authenticate the initial request, return its matching reply, and retain correlation.
    ///
    /// # Errors
    /// Preserve dispatch, budget, entropy and packet refusals.
    fn initial_exchange(
        &self,
        budget: &Budget,
        c2s: &[u8; 32],
        s2c: &[u8; 32],
        fill: impl FnOnce(&mut [u8; 16]) -> Result<(), Failure>,
        now: &impl Fn() -> Instant,
    ) -> Result<Correlation, Failure> {
        let (request, sender): (Zeroizing<[u8; REQUEST_BYTES + 1]>, SocketAddrV4) =
            self.receive(budget, now)?;
        let mut nonce: Zeroizing<[u8; 16]> = Zeroizing::new([0; 16]);
        budget.poll(now())?;
        fill(&mut nonce)?;
        budget.poll(now())?;
        let (initial, reply): (Correlation, Zeroizing<[u8; REPLY_BYTES]>) =
            admit_and_seal(&request[..REQUEST_BYTES], c2s, s2c, &nonce)
                .map_err(Failure::Datagram)?;
        self.send_reply(budget, now, sender, &reply[..])?;
        Ok(initial)
    }

    /// Consume one bounded datagram from this listener's IPv4 loopback.
    ///
    /// # Errors
    /// Refuse a bad endpoint or extent, expired deadline or failed read.
    fn receive(
        &self,
        budget: &Budget,
        now: &impl Fn() -> Instant,
    ) -> Result<(Zeroizing<[u8; REQUEST_BYTES + 1]>, SocketAddrV4), Failure> {
        // A one-byte sentinel distinguishes an overlong truncated datagram from
        // an exact frame on systems that return the truncated byte count.
        let mut request: Zeroizing<[u8; REQUEST_BYTES + 1]> =
            Zeroizing::new([0; REQUEST_BYTES + 1]);
        let (length, sender): (usize, SocketAddr) = loop {
            let timeout: Duration = budget.poll(now())?;
            self.socket
                .set_read_timeout(Some(timeout))
                .map_err(|error| Failure::Io(error.kind()))?;
            budget.poll(now())?;
            match self.socket.recv_from(request.as_mut()) {
                Ok(received) => break received,
                Err(error)
                    if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
                #[cfg(windows)]
                Err(error) if error.raw_os_error() == Some(WSAEMSGSIZE) => {
                    return Err(Failure::Datagram(PacketFailure::Length));
                }
                Err(error) => return Err(Failure::Io(error.kind())),
            }
        };
        budget.poll(now())?;

        let SocketAddr::V4(sender) = sender else {
            return Err(Failure::Arguments);
        };
        if *sender.ip() != Ipv4Addr::LOCALHOST || sender.port() == 0 {
            return Err(Failure::Arguments);
        }
        if length != REQUEST_BYTES {
            return Err(Failure::Datagram(PacketFailure::Length));
        }

        Ok((request, sender))
    }

    /// Attempt one nonblocking send to the endpoint that supplied this request.
    ///
    /// # Errors
    /// Preserve an expired deadline, failed send, partial send or late outcome.
    fn send_reply(
        &self,
        budget: &Budget,
        now: &impl Fn() -> Instant,
        sender: SocketAddrV4,
        reply: &[u8],
    ) -> Result<(), Failure> {
        budget.poll(now())?;
        self.socket
            .set_nonblocking(true)
            .map_err(|error| Failure::Io(error.kind()))?;
        budget.poll(now())?;
        let sent: usize = self
            .socket
            .send_to(reply, sender)
            .map_err(|error| Failure::Io(error.kind()))?;
        if sent != reply.len() {
            return Err(Failure::Io(ErrorKind::WriteZero));
        }
        budget.poll(now())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
