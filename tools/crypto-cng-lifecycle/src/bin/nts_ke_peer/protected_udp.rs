// SPDX-License-Identifier: GPL-2.0-only
//! One owned IPv4 loopback socket for the standalone protected peer.

use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::num::NonZeroU16;
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::{
    Budget, Failure,
    protected_packet::{PacketFailure, REQUEST_BYTES, admit_and_seal},
};

/// Windows reports an overlong UDP datagram as a receive error.
#[cfg(windows)]
const WSAEMSGSIZE: i32 = 10_040;

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

        let mut nonce: Zeroizing<[u8; 16]> = Zeroizing::new([0; 16]);
        budget.poll(now())?;
        fill(&mut nonce)?;
        budget.poll(now())?;
        let reply: Zeroizing<[u8; 140]> =
            admit_and_seal(&request[..length], c2s, s2c, &nonce).map_err(Failure::Datagram)?;
        budget.poll(now())?;

        self.socket
            .set_nonblocking(true)
            .map_err(|error| Failure::Io(error.kind()))?;
        budget.poll(now())?;
        let sent: usize = self
            .socket
            .send_to(&reply[..], sender)
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
