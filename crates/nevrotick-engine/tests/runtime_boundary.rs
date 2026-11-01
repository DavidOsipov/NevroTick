//! Real CLI controls for the bounded transport boundary.
//!
//! CLI controls send literal packets only over
//! owned IPv4/IPv6 loopback and execute Cargo's current worker-built entrypoint.
//! No clock/service changes occur. Port release and finite process observations
//! are narrower than complete native I/O, resource or Windows qualification.

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Keep two distinct controlled peers alive while testing actual native setup.
///
/// # Errors
/// Return host socket setup errors without substituting a protocol outcome.
fn controlled_peers() -> Result<([UdpSocket; 2], [SocketAddr; 2]), Box<dyn Error>> {
    let first = UdpSocket::bind("127.0.0.1:0")?;
    let second = UdpSocket::bind("127.0.0.1:0")?;
    let endpoints = [first.local_addr()?, second.local_addr()?];
    Ok(([first, second], endpoints))
}

/// Bounded fixture result; OS/setup errors never become protocol success.
type Check<T> = Result<T, Box<dyn Error>>;
/// Maximum captured bytes per stream; file-size polling is sampling, not a quota.
const OUTPUT_LIMIT: u64 = 16_384;
/// Whole-child observation budget, including ordinary startup.
const CASE_WALL: Duration = Duration::from_secs(3);
/// Bound cleanup polling after an exact owned process is killed.
const REAP_WALL: Duration = Duration::from_millis(500);
/// Distinguish exclusive fixture directory names inside this test process.
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// Return the independent failed fact as a test error.
///
/// # Errors
/// Return the supplied fixed fact when the independent condition is false.
fn check(condition: bool, fact: &'static str) -> io::Result<()> {
    if condition {
        Ok(())
    } else {
        Err(io::Error::other(fact))
    }
}

/// Select a writable capture or a verified broken-pipe standard sink.
#[derive(Clone, Copy)]
enum Sinks {
    /// Capture both streams normally.
    Writable,
    /// Exercise the actual bounded diagnostic-output failure path.
    BrokenStdout,
    /// Exercise the final stderr-write failure exit without a mock writer.
    BrokenStderr,
}

/// Close the sole reader and verify the writer's actual failure before spawning.
/// No reader thread or read handle can mask the child's intended sink failure.
///
/// # Errors
/// Preserve pipe creation failure; require a write error of kind `BrokenPipe`.
fn broken_pipe_sink() -> io::Result<Stdio> {
    let (reader, mut writer) = io::pipe()?;
    drop(reader);
    let failure = writer
        .write_all(b"sink prerequisite")
        .err()
        .ok_or_else(|| io::Error::other("closed-reader pipe accepted fixture bytes"))?;
    check(
        failure.kind() == io::ErrorKind::BrokenPipe,
        "fixture sink did not produce BrokenPipe",
    )?;
    Ok(Stdio::from(writer))
}

/// Actual child result captured after it has exited and before fixture removal.
#[derive(Debug)]
struct CliOutput {
    /// Actual native process status, never inferred from transcript text.
    status: ExitStatus,
    /// Bounded actual stdout bytes.
    stdout: Vec<u8>,
    /// Bounded actual stderr bytes.
    stderr: Vec<u8>,
}

/// Single exact child plus its exclusively created output directory.
///
/// File-backed capture avoids detached pipe-reader threads. The fixed CLI has no
/// descendant-process surface. Poll file sizes/deadlines; on failure kill and
/// poll-reap only this `Child` before removing any owned capture. A cleanup error
/// fails the case; the outer configured PID namespace remains final custody.
struct CliChild {
    /// The Cargo-built process; its handle remains owned until actual exit.
    child: Option<Child>,
    /// Newly created fixture directory, never a caller-provided deletion path.
    directory: PathBuf,
    /// Whole-child observation origin.
    started: Instant,
    /// Successful explicit cleanup suppresses redundant fallback work.
    cleaned: bool,
}

impl CliChild {
    /// Spawn Cargo's actual current worker image with separate literal arguments.
    ///
    /// # Errors
    /// Preserve directory, file, process-start or handle setup failures.
    fn spawn(arguments: &[OsString], sinks: Sinks) -> Check<Self> {
        let serial = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "nevrotick-cargo-cli-{}-{serial}",
            std::process::id()
        ));
        let builder = fs::DirBuilder::new();
        #[cfg(unix)]
        let builder = {
            use std::os::unix::fs::DirBuilderExt;
            let mut builder = builder;
            builder.mode(0o700);
            builder
        };
        builder.create(&directory)?;
        let mut owned = Self {
            child: None,
            directory,
            started: Instant::now(),
            cleaned: false,
        };
        let output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(owned.directory.join("stdout"))?;
        let error = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(owned.directory.join("stderr"))?;
        let output = if matches!(sinks, Sinks::BrokenStdout) {
            broken_pipe_sink()?
        } else {
            Stdio::from(output)
        };
        let error = if matches!(sinks, Sinks::BrokenStderr) {
            broken_pipe_sink()?
        } else {
            Stdio::from(error)
        };
        owned.child = Some(
            Command::new(env!("CARGO_BIN_EXE_nevrotick-observe"))
                .args(arguments)
                .current_dir(&owned.directory)
                .stdin(Stdio::null())
                .stdout(output)
                .stderr(error)
                .spawn()?,
        );
        Ok(owned)
    }

    /// Poll one exact child and sampled capture budgets without blocking on pipes.
    ///
    /// # Errors
    /// Reject timeout/output overflow or preserve native wait/metadata errors.
    fn poll(&mut self) -> Check<Option<ExitStatus>> {
        check(
            self.started.elapsed() < CASE_WALL,
            "CLI whole-child deadline exceeded",
        )?;
        for name in ["stdout", "stderr"] {
            check(
                fs::metadata(self.directory.join(name))?.len() <= OUTPUT_LIMIT,
                "CLI sampled output cap exceeded",
            )?;
        }
        Ok(self
            .child
            .as_mut()
            .ok_or_else(|| io::Error::other("missing owned CLI"))?
            .try_wait()?)
    }

    /// Observe termination within a finite cleanup wait before releasing captures.
    ///
    /// # Errors
    /// Preserve kill/reap failures instead of claiming successful teardown.
    fn stop(&mut self) -> io::Result<()> {
        let Some(child) = self.child.as_mut() else {
            return Ok(());
        };
        if child.try_wait()?.is_some() {
            return Ok(());
        }
        if let Err(error) = child.kill()
            && child.try_wait()?.is_none()
        {
            return Err(error);
        }
        let began = Instant::now();
        loop {
            if child.try_wait()?.is_some() {
                return Ok(());
            }
            check(
                began.elapsed() < REAP_WALL,
                "owned CLI did not reap within cleanup budget",
            )?;
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    /// Remove only the two owned files and their now-empty directory after reap.
    ///
    /// # Errors
    /// Preserve any ownership/cleanup problem; no recursive or wildcard deletion.
    fn cleanup(&mut self) -> io::Result<()> {
        self.stop()?;
        self.child = None;
        for name in ["stdout", "stderr"] {
            let path = self.directory.join(name);
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        fs::remove_dir(&self.directory)?;
        self.cleaned = true;
        Ok(())
    }

    /// Read bounded bytes only after actual exit, then explicitly close custody.
    ///
    /// # Errors
    /// Report primary capture failure and any additional cleanup failure together.
    fn finish(mut self) -> Check<CliOutput> {
        let output: Check<CliOutput> = (|| {
            let status = self
                .poll()?
                .ok_or_else(|| io::Error::other("CLI still running at finish"))?;
            let read = |name| -> Check<Vec<u8>> {
                let mut bytes = Vec::new();
                File::open(self.directory.join(name))?
                    .take(OUTPUT_LIMIT + 1)
                    .read_to_end(&mut bytes)?;
                check(
                    u64::try_from(bytes.len())? <= OUTPUT_LIMIT,
                    "captured CLI output exceeds bound",
                )?;
                Ok(bytes)
            };
            Ok(CliOutput {
                status,
                stdout: read("stdout")?,
                stderr: read("stderr")?,
            })
        })();
        let cleanup = self.cleanup();
        match (output, cleanup) {
            (Ok(output), Ok(())) => Ok(output),
            (Err(error), Ok(())) => Err(error),
            (Ok(_), Err(error)) => Err(error.into()),
            (Err(primary), Err(cleanup)) => {
                Err(format!("{primary}; CLI cleanup also failed: {cleanup}").into())
            }
        }
    }
}

impl Drop for CliChild {
    /// Bounded fallback on a failing path; never leave an unbounded reader thread.
    fn drop(&mut self) {
        if !self.cleaned
            && let Err(error) = self.cleanup()
        {
            eprintln!("owned CLI cleanup incomplete: {error}");
        }
    }
}

/// Finite independent entrypoint scenarios; no production value supplies truth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CliCase {
    /// Two independently correlated ordinary headers.
    Healthy,
    /// Wrong origin, two oversized datagrams and wrong source precede a valid neighbor.
    RejectedThenHealthy,
    /// Literal stratum-zero control information stays unauthenticated/ineligible.
    Control,
    /// No response reaches either outstanding request.
    Timeout,
    /// Stop must remain responsive while slot zero receives invalid traffic.
    Stop,
    /// Current generation invalidation retires both unanswered requests.
    Invalidate,
}

impl CliCase {
    /// Literal selected terminal labels for both slots.
    fn statuses(self) -> [&'static str; 2] {
        match self {
            Self::Healthy | Self::RejectedThenHealthy => ["header", "header"],
            Self::Control => ["control", "control"],
            Self::Timeout => ["timeout", "timeout"],
            Self::Stop => ["stopped", "header"],
            Self::Invalidate => ["stale", "stale"],
        }
    }

    /// Build only the independently selected numeric laboratory arguments.
    fn arguments(self, endpoints: [SocketAddr; 2]) -> Vec<OsString> {
        let timeout = match self {
            Self::Timeout => "120",
            Self::Stop => "1500",
            _ => "700",
        };
        let mut args = vec![
            "--peer".into(),
            endpoints[0].to_string().into(),
            "--peer".into(),
            endpoints[1].to_string().into(),
            "--timeout-ms".into(),
            timeout.into(),
        ];
        match self {
            Self::Stop => args.extend(["--stop-after-ms".into(), "150".into()]),
            Self::Invalidate => args.extend(["--invalidate-after-ms".into(), "100".into()]),
            _ => {}
        }
        args
    }
}

/// One bounded explicitly scheduled datagram, attempted once without implicit retry.
struct ReplyPlan {
    /// Owned sending peer; index two is the deliberately wrong source.
    sender: usize,
    /// Actual observed client endpoint receiving this datagram.
    client: SocketAddr,
    /// Independent reply bytes with only the observed correlation copied in.
    bytes: Vec<u8>,
    /// Monotonic fixture schedule, not a product timestamp oracle.
    due: Instant,
    /// No failed send is retried as an unrecorded extra operation.
    attempted: bool,
}

/// Physical observations and finite outgoing actions for one CLI process.
struct CliTraffic {
    /// One actual query per configured peer; duplicates fail immediately.
    requests: [Option<[u8; 48]>; 2],
    /// The actual independently observed client sockets.
    clients: [Option<SocketAddr>; 2],
    /// At most six finite ordinary/hostile reply actions.
    plans: Vec<ReplyPlan>,
    /// Bound prior-send ICMP accounting independently of successful packets.
    icmp: usize,
    /// Actual successful sends per owned peer; errors do not increment these.
    sent: [usize; 3],
    /// Invalid stop traffic has a finite attempt budget and ongoing schedule.
    flood_attempts: usize,
    /// Next invalid-traffic send time once slot zero has queried.
    flood_due: Option<Instant>,
}

/// Construct the independently declared48-byte packet; copy only the live nonce.
fn literal_reply(query: &[u8; 48], stratum: u8) -> Vec<u8> {
    let mut reply = vec![0; 48];
    reply[0] = 0x24;
    reply[1] = stratum;
    if stratum == 0 {
        reply[12..16].copy_from_slice(b"RATE");
    }
    reply[24..32].copy_from_slice(&query[40..48]);
    reply[32..40].copy_from_slice(&[0xe0, 0x12, 0x34, 0x56, 0x10, 0, 0, 0]);
    reply[40..48].copy_from_slice(&[0xe0, 0x12, 0x34, 0x56, 0x20, 0, 0, 0]);
    reply
}

impl CliTraffic {
    /// Initialize empty observations for one process, with no hidden retries.
    fn new() -> Self {
        Self {
            requests: [None; 2],
            clients: [None; 2],
            plans: Vec::new(),
            icmp: 0,
            sent: [0; 3],
            flood_attempts: 0,
            flood_due: None,
        }
    }

    /// Record one actual query before planning any reply.
    ///
    /// # Errors
    /// Reject malformed/duplicate requests and any source outside the owned family.
    fn query(
        &mut self,
        case: CliCase,
        slot: usize,
        packet: &[u8],
        client: SocketAddr,
        ip: IpAddr,
    ) -> Check<()> {
        check(slot < 2, "fixture query slot exceeds the two associations")?;
        check(
            self.requests[slot].is_none(),
            "CLI emitted a retry/duplicate query",
        )?;
        check(
            packet.len() == 48 && packet[0] == 0x23 && packet[1..40].iter().all(|b| *b == 0),
            "actual query differs from literal minimal NTPv4 request",
        )?;
        check(
            client.ip() == ip && client.port() != 0,
            "query source differs from owned loopback family",
        )?;
        let query: [u8; 48] = packet.try_into()?;
        check(
            query[40..48].iter().any(|b| *b != 0),
            "actual correlation token is zero",
        )?;
        self.requests[slot] = Some(query);
        self.clients[slot] = Some(client);
        let now = Instant::now();
        let reply = literal_reply(&query, [1, 2][slot]);
        match case {
            CliCase::Timeout | CliCase::Invalidate => {}
            CliCase::Stop if slot == 0 => self.flood_due = Some(now),
            CliCase::Control => self.plan(slot, client, literal_reply(&query, 0), now)?,
            CliCase::RejectedThenHealthy if slot == 0 => {
                let mut wrong_origin = reply.clone();
                wrong_origin[24..32].fill(0);
                self.plan(0, client, wrong_origin, now)?;
                let mut oversized = reply.clone();
                oversized.push(0);
                self.plan(0, client, oversized, now + Duration::from_millis(20))?;
                let mut large = reply.clone();
                large.resize(65_000, 0);
                self.plan(0, client, large, now + Duration::from_millis(40))?;
                self.plan(
                    2,
                    client,
                    literal_reply(&query, 3),
                    now + Duration::from_millis(60),
                )?;
                self.plan(0, client, reply, now + Duration::from_millis(80))?;
            }
            _ => self.plan(slot, client, reply, now)?,
        }
        Ok(())
    }

    /// Add one bounded datagram action.
    ///
    /// # Errors
    /// Reject unexpected fixture action growth before allocating another entry.
    fn plan(
        &mut self,
        sender: usize,
        client: SocketAddr,
        bytes: Vec<u8>,
        due: Instant,
    ) -> io::Result<()> {
        check(
            self.plans.len() < 6 && sender < 3 && bytes.len() <= 65_000,
            "reply plan exceeded finite action/peer/byte budget",
        )?;
        self.plans.push(ReplyPlan {
            sender,
            client,
            bytes,
            due,
            attempted: false,
        });
        Ok(())
    }

    /// Receive at most one query per peer in this iteration, separating prior-send ICMP.
    ///
    /// # Errors
    /// Preserve unknown OS errors; exactly10054 requires a prior own send and budget.
    fn receive(&mut self, case: CliCase, peers: &[UdpSocket; 3], ip: IpAddr) -> Check<()> {
        for (slot, peer) in peers[..2].iter().enumerate() {
            let mut bytes = [0; 49];
            match peer.recv_from(&mut bytes) {
                Ok((length, client)) => self.query(case, slot, &bytes[..length], client, ip)?,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                Err(error) if error.raw_os_error() == Some(10054) && self.sent[slot] > 0 => {
                    self.icmp += 1;
                    check(self.icmp <= 8, "peer ICMP observation budget exceeded")?;
                }
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }

    /// Attempt each due datagram once and keep invalid traffic scheduled until exit.
    ///
    /// # Errors
    /// Preserve send failures and reject stop traffic exhausting its48-attempt bound.
    fn send(&mut self, case: CliCase, peers: &[UdpSocket; 3]) -> Check<()> {
        for plan in &mut self.plans {
            if !plan.attempted && Instant::now() >= plan.due {
                plan.attempted = true;
                match peers[plan.sender].send_to(&plan.bytes, plan.client) {
                    Ok(length) => {
                        check(
                            length == plan.bytes.len(),
                            "owned peer sent a partial reply",
                        )?;
                        self.sent[plan.sender] += 1;
                    }
                    Err(error)
                        if error.raw_os_error() == Some(10054) && self.sent[plan.sender] > 0 =>
                    {
                        self.icmp += 1;
                        check(self.icmp <= 8, "peer ICMP observation budget exceeded")?;
                    }
                    Err(error) => return Err(error.into()),
                }
            }
        }
        if case == CliCase::Stop && self.flood_due.is_some_and(|due| Instant::now() >= due) {
            check(
                self.flood_attempts < 48,
                "stop traffic cap exhausted while CLI remained alive",
            )?;
            self.flood_attempts += 1;
            let client =
                self.clients[0].ok_or_else(|| io::Error::other("stop flood lacks actual query"))?;
            match peers[0].send_to(&[0; 49], client) {
                Ok(length) => {
                    check(length == 49, "invalid stop datagram was partial")?;
                    self.sent[0] += 1;
                }
                Err(error) if error.raw_os_error() == Some(10054) && self.sent[0] > 0 => {
                    self.icmp += 1;
                    check(self.icmp <= 8, "peer ICMP observation budget exceeded")?;
                }
                Err(error) => return Err(error.into()),
            }
            self.flood_due = Some(Instant::now() + Duration::from_millis(10));
        }
        Ok(())
    }
}

/// Decode exactly the independently declared bounded ASCII field set.
///
/// # Errors
/// Reject duplicate/unknown/missing fields, controls and unexpected line lengths.
fn fields<'a>(line: &'a str, expected: &[&str]) -> Check<BTreeMap<&'a str, &'a str>> {
    let line = line.strip_suffix('\r').unwrap_or(line);
    check(
        !line.is_empty()
            && line.len() <= 256
            && line.bytes().all(|b| b.is_ascii_graphic() || b == b' '),
        "CLI record is not bounded printable ASCII",
    )?;
    let mut values = BTreeMap::new();
    for token in line.split(' ') {
        let (key, value) = token
            .split_once('=')
            .ok_or_else(|| io::Error::other("malformed CLI field"))?;
        check(
            !key.is_empty() && !value.is_empty() && values.insert(key, value).is_none(),
            "empty/duplicate CLI field",
        )?;
    }
    check(
        values.len() == expected.len() && expected.iter().all(|key| values.contains_key(key)),
        "CLI field set differs from independent record contract",
    )?;
    Ok(values)
}

/// Read a canonical unsigned diagnostic integer without a production parser.
///
/// # Errors
/// Reject missing, signed, nondecimal, padded or overflowing values.
fn integer(values: &BTreeMap<&str, &str>, key: &str) -> Check<u128> {
    let raw = values
        .get(key)
        .ok_or_else(|| io::Error::other("missing CLI integer"))?;
    check(
        raw.len() <= 40
            && raw.bytes().all(|b| b.is_ascii_digit())
            && (*raw == "0" || !raw.starts_with('0')),
        "CLI integer is not canonical unsigned decimal",
    )?;
    Ok(raw.parse()?)
}

/// Check each independently promised result/counter/trust fact for a slot.
///
/// # Errors
/// Return the first mismatching literal status, count, payload or trust fact.
fn result_record(line: &str, case: CliCase, slot: usize) -> Check<()> {
    check(
        slot < 2,
        "result observer slot exceeds the two associations",
    )?;
    let status = case.statuses()[slot];
    let mut expected = vec![
        "event",
        "slot",
        "status",
        "sent",
        "received",
        "rejected",
        "authentication",
        "eligible",
    ];
    if status == "header" {
        expected.push("stratum");
    }
    let values = fields(line, &expected)?;
    check(
        values["event"] == "result"
            && integer(&values, "slot")? == u128::try_from(slot)?
            && values["status"] == status,
        "CLI result slot/status differs from literal scenario",
    )?;
    check(
        values["authentication"] == "none" && values["eligible"] == "0" && values["sent"] == "1",
        "CLI authenticated/eligible/sent labels differ",
    )?;
    let received = integer(&values, "received")?;
    let rejected = integer(&values, "rejected")?;
    check(
        received <= 64 && rejected <= 64,
        "CLI counters exceed receive laboratory budget",
    )?;
    if case == CliCase::RejectedThenHealthy && slot == 0 {
        check(
            received >= 1 && rejected >= 4,
            "hostile packets lost rejection history or valid neighbor",
        )?;
    } else if case == CliCase::Stop && slot == 0 {
        check(
            rejected >= 1,
            "stop case observed no invalid traffic rejection",
        )?;
    } else if matches!(status, "timeout" | "stale") {
        check(
            received == 0 && rejected == 0,
            "unanswered request invented receive history",
        )?;
    } else {
        check(
            received == 1 && rejected == 0,
            "single literal reply changed receive/rejection history",
        )?;
    }
    if status == "header" {
        check(
            integer(&values, "stratum")? == [1, 2][slot],
            "CLI stratum differs from independent reply",
        )?;
    }
    Ok(())
}

/// Check the actual five-record stream against observed sockets and literal cases.
///
/// # Errors
/// Reject nonzero exit/stderr, partial/extra output and any changed promised fact.
fn transcript(output: &CliOutput, case: CliCase, clients: [SocketAddr; 2]) -> Check<()> {
    check(
        output.status.success() && output.stderr.is_empty(),
        "CLI failed or wrote unexpected stderr",
    )?;
    check(output.stdout.is_ascii(), "CLI stdout is not actual ASCII")?;
    let text = std::str::from_utf8(&output.stdout)?;
    let text = text
        .strip_suffix('\n')
        .ok_or_else(|| io::Error::other("unterminated CLI stream"))?;
    let lines: Vec<_> = text.split('\n').collect();
    check(
        lines.len() == 5,
        "CLI stream does not have exactly five records",
    )?;
    for (slot, client) in clients.iter().enumerate() {
        let bound = fields(lines[slot], &["event", "slot", "local"])?;
        check(
            bound["event"] == "bound"
                && integer(&bound, "slot")? == u128::try_from(slot)?
                && bound["local"].parse::<SocketAddr>()? == *client,
            "bound record differs from physical request source",
        )?;
        result_record(lines[slot + 2], case, slot)?;
    }
    let done = fields(lines[4], &["event", "elapsed_ns"])?;
    let elapsed = integer(&done, "elapsed_ns")?;
    check(
        done["event"] == "done" && elapsed < CASE_WALL.as_nanos(),
        "CLI completion record is invalid/outside budget",
    )?;
    if case == CliCase::Timeout {
        check(
            elapsed >= 120_000_000,
            "CLI timeout elapsed precedes the requested 120ms lifetime",
        )?;
    }
    Ok(())
}

/// Challenge the observer with wrong, missing and unexpected delivered facts.
///
/// # Errors
/// Require the intended rejection after the same actual healthy stream passed.
fn transcript_controls(output: &CliOutput, clients: [SocketAddr; 2]) -> Check<()> {
    let healthy = std::str::from_utf8(&output.stdout)?;
    let faults = [
        (
            healthy.replace("authentication=none", "authentication=nts"),
            "CLI authenticated/eligible/sent labels differ",
        ),
        (
            healthy.replace(" eligible=0", ""),
            "CLI field set differs from independent record contract",
        ),
        (
            format!("{healthy}event=done elapsed_ns=1\n"),
            "CLI stream does not have exactly five records",
        ),
    ];
    for (text, diagnostic) in faults {
        let changed = CliOutput {
            status: output.status,
            stdout: text.into_bytes(),
            stderr: Vec::new(),
        };
        match transcript(&changed, CliCase::Healthy, clients) {
            Ok(()) => {
                return Err(
                    "CLI observer admitted an independently authored faulty transcript".into(),
                );
            }
            Err(error) => check(
                error.to_string() == diagnostic,
                "CLI observer rejected for an unrelated reason",
            )?,
        }
    }
    Ok(())
}

/// A completed unanswered 120ms query cannot report zero or one nanosecond.
/// The original actual timeout stream is the positive control before alteration.
///
/// # Errors
/// Reject missing completion records or a fault rejected for an unrelated reason.
fn timeout_elapsed_controls(output: &CliOutput, clients: [SocketAddr; 2]) -> Check<()> {
    let healthy = std::str::from_utf8(&output.stdout)?;
    let (prefix, _) = healthy
        .rsplit_once("event=done elapsed_ns=")
        .ok_or_else(|| io::Error::other("actual timeout completion record absent"))?;
    for value in ["0", "1"] {
        let changed = CliOutput {
            status: output.status,
            stdout: format!("{prefix}event=done elapsed_ns={value}\n").into_bytes(),
            stderr: Vec::new(),
        };
        match transcript(&changed, CliCase::Timeout, clients) {
            Ok(()) => {
                return Err(io::Error::other(
                    "CLI observer admitted an impossible timeout elapsed value",
                )
                .into());
            }
            Err(error) => check(
                error.to_string() == "CLI timeout elapsed precedes the requested 120ms lifetime",
                "timeout elapsed control failed for an unrelated reason",
            )?,
        }
    }
    Ok(())
}

/// Exercise one real executable against owned peers, then probe release before teardown.
///
/// # Errors
/// Preserve setup/I/O/cleanup errors and independent packet/transcript failures.
fn cli_case(ip: IpAddr, case: CliCase) -> Check<()> {
    let peers = [
        UdpSocket::bind(SocketAddr::new(ip, 0))?,
        UdpSocket::bind(SocketAddr::new(ip, 0))?,
        UdpSocket::bind(SocketAddr::new(ip, 0))?,
    ];
    for peer in &peers {
        peer.set_nonblocking(true)?;
    }
    let endpoints = [peers[0].local_addr()?, peers[1].local_addr()?];
    let mut child = CliChild::spawn(&case.arguments(endpoints), Sinks::Writable)?;
    let mut traffic = CliTraffic::new();
    let mut exit_seen = None;
    loop {
        if child.poll()?.is_some() && exit_seen.is_none() {
            exit_seen = Some(Instant::now());
        }
        traffic.receive(case, &peers, ip)?;
        if exit_seen.is_none() {
            traffic.send(case, &peers)?;
        }
        if exit_seen.is_some_and(|at: Instant| at.elapsed() >= Duration::from_millis(20)) {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let output = child.finish()?;
    let clients = [
        traffic.clients[0].ok_or_else(|| io::Error::other("missing slot0 query"))?,
        traffic.clients[1].ok_or_else(|| io::Error::other("missing slot1 query"))?,
    ];
    check(
        clients[0] != clients[1] && traffic.requests[0].is_some() && traffic.requests[1].is_some(),
        "CLI did not use two distinct actual association sockets",
    )?;
    let first = traffic.requests[0].ok_or_else(|| io::Error::other("missing first query"))?;
    let second = traffic.requests[1].ok_or_else(|| io::Error::other("missing second query"))?;
    check(
        first[40..48] != second[40..48],
        "actual association tokens collided",
    )?;
    if let Err(error) = transcript(&output, case, clients) {
        return Err(format!("{case:?}/{ip}: {error}; actual={output:?}").into());
    }
    if case == CliCase::Healthy {
        transcript_controls(&output, clients)?;
    }
    if case == CliCase::Timeout {
        timeout_elapsed_controls(&output, clients)?;
    }
    // The exact child has exited; ALL peer sockets are still owned/open here.
    let _first_released = UdpSocket::bind(clients[0])?;
    let _second_released = UdpSocket::bind(clients[1])?;
    Ok(())
}

/// The mutation runner must execute its own built CLI, not a baseline-image oracle.
///
/// # Errors
/// Preserve independent scenario, physical packet, process and resource failures.
#[test]
fn cli_worker_binary_preserves_literal_transport_contracts() -> Check<()> {
    for ip in [
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
    ] {
        for case in [
            CliCase::Healthy,
            CliCase::RejectedThenHealthy,
            CliCase::Control,
            CliCase::Timeout,
            CliCase::Stop,
            CliCase::Invalidate,
        ] {
            cli_case(ip, case)?;
        }
    }
    Ok(())
}

/// Wait for one bounded argument/sink case and return actual captured native output.
///
/// # Errors
/// Preserve native setup, timeout, capture and cleanup failures.
fn run_cli(arguments: &[&str], sinks: Sinks) -> Check<CliOutput> {
    let args: Vec<_> = arguments.iter().map(OsString::from).collect();
    run_os_cli(&args, sinks)
}

/// Preserve raw operating-system argument bytes in the actual executable call.
///
/// # Errors
/// Keep startup, bounded wait, output and cleanup failures visible.
fn run_os_cli(arguments: &[OsString], sinks: Sinks) -> Check<CliOutput> {
    let mut child = CliChild::spawn(arguments, sinks)?;
    while child.poll()?.is_none() {
        std::thread::sleep(Duration::from_millis(1));
    }
    child.finish()
}

/// Independent malformed/unauthorized argument facts; no production error mapper.
const ARGUMENT_REJECTIONS: &[(&[&str], &str)] = &[
    (&[], "invalid_arguments"),
    (&["--peer"], "invalid_arguments"),
    (&["--peer", "not-an-endpoint"], "invalid_endpoint"),
    (
        &[
            "--peer",
            "127.0.0.1:1",
            "--peer",
            "127.0.0.1:2",
            "--timeout-ms",
            "-1",
        ],
        "invalid_arguments",
    ),
    (
        &[
            "--peer",
            "127.0.0.1:1",
            "--peer",
            "127.0.0.1:2",
            "--timeout-ms",
            "0",
        ],
        "invalid_timeout",
    ),
    (
        &[
            "--peer",
            "127.0.0.1:1",
            "--peer",
            "127.0.0.1:2",
            "--timeout-ms",
            "30001",
        ],
        "invalid_timeout",
    ),
    (
        &[
            "--peer",
            "192.0.2.1:123",
            "--peer",
            "127.0.0.1:2",
            "--timeout-ms",
            "700",
        ],
        "invalid_peer",
    ),
    (
        &[
            "--peer",
            "127.0.0.1:0",
            "--peer",
            "127.0.0.1:2",
            "--timeout-ms",
            "700",
        ],
        "invalid_peer",
    ),
    (
        &[
            "--peer",
            "127.0.0.1:1",
            "--peer",
            "127.0.0.1:1",
            "--timeout-ms",
            "700",
        ],
        "duplicate_peer",
    ),
];

/// Check one exact argument rejection through the real current CLI.
///
/// # Errors
/// Preserve setup/cleanup failures and reject changed error labels or exit codes.
fn argument_rejection(arguments: &[&str], code: &str) -> Check<()> {
    let output = run_cli(arguments, Sinks::Writable)?;
    check(
        output.status.code() == Some(2)
            && output.stdout.is_empty()
            && output.stderr == format!("error={code}\n").as_bytes(),
        "CLI rejection differs from literal argument contract",
    )?;
    Ok(())
}

/// OS-native non-text values must take the real argument error path.
///
/// # Errors
/// Preserve OS failures and reject any leaked raw input or changed error/exit.
#[cfg(unix)]
fn non_text_arguments() -> Check<()> {
    use std::os::unix::ffi::OsStringExt;
    for arguments in [
        vec![OsString::from_vec(vec![0xff]), "1".into()],
        vec!["--peer".into(), OsString::from_vec(vec![0xff])],
    ] {
        let output = run_os_cli(&arguments, Sinks::Writable)?;
        check(
            output.status.code() == Some(2)
                && output.stdout.is_empty()
                && output.stderr == b"error=invalid_arguments\n",
            "CLI did not reject actual non-text argument bytes safely",
        )?;
    }
    Ok(())
}

/// Exact duration/value boundaries and duplicate controls retain their error owner.
/// All numeric peers used by a wrongly admitted child remain parent-owned loopback
/// endpoints; no public or ambient service is used by a fault variant.
///
/// # Errors
/// Preserve peer setup/child cleanup failures and reject changed exit/error labels.
fn cli_argument_boundary_rejections() -> Check<()> {
    let (_peers, endpoints) = controlled_peers()?;
    let first = endpoints[0].to_string();
    let second = endpoints[1].to_string();
    let base = [
        "--peer",
        first.as_str(),
        "--peer",
        second.as_str(),
        "--timeout-ms",
        "700",
    ];
    for extra in [
        &["--peer", "not-an-endpoint"][..],
        &["--stop-after-ms", "0", "--stop-after-ms", "0"][..],
        &["--invalidate-after-ms", "0", "--invalidate-after-ms", "0"][..],
    ] {
        let mut arguments = base.to_vec();
        arguments.extend_from_slice(extra);
        argument_rejection(&arguments, "invalid_arguments")?;
    }
    argument_rejection(
        &[
            "--peer",
            first.as_str(),
            "--peer",
            second.as_str(),
            "--timeout-ms",
            "99999999",
        ],
        "invalid_timeout",
    )?;
    let exact_value_limit = "0".repeat(128);
    argument_rejection(&["--peer", &exact_value_limit], "invalid_endpoint")?;
    Ok(())
}

/// Observe literal argument rejections and real output-sink failure exits.
///
/// # Errors
/// Reject wrong native exit, error-code disclosure or unexpected partial output.
#[test]
fn cli_worker_binary_rejects_arguments_and_observes_sink_failures() -> Check<()> {
    cli_argument_boundary_rejections()?;
    let base = [
        "--peer",
        "127.0.0.1:10001",
        "--peer",
        "127.0.0.1:10002",
        "--timeout-ms",
        "700",
    ];
    for (arguments, code) in ARGUMENT_REJECTIONS {
        argument_rejection(arguments, code)?;
    }
    for extra in [
        ["--timeout-ms", "1"],
        ["--unknown", "1"],
        ["--peer", "127.0.0.1:3"],
        ["--stop-after-ms", "x"],
    ] {
        let mut args = base.to_vec();
        args.extend(extra);
        let output = run_cli(&args, Sinks::Writable)?;
        check(
            output.status.code() == Some(2)
                && output.stdout.is_empty()
                && output.stderr == b"error=invalid_arguments\n",
            "CLI repeated/unknown option was not rejected exactly",
        )?;
    }
    let too_long = "x".repeat(129);
    argument_rejection(&["--peer", &too_long], "invalid_arguments")?;
    let too_long_option = "x".repeat(33);
    argument_rejection(&[&too_long_option, "1"], "invalid_arguments")?;
    argument_rejection(
        &[
            "--peer",
            "127.0.0.1:1",
            "--peer",
            "127.0.0.1:2",
            "--timeout-ms",
            "999999999",
        ],
        "invalid_arguments",
    )?;
    #[cfg(unix)]
    non_text_arguments()?;
    let (_peers, endpoints) = controlled_peers()?;
    let mut valid = CliCase::Healthy.arguments(endpoints);
    valid.extend([
        "--stop-after-ms".into(),
        "0".into(),
        "--invalidate-after-ms".into(),
        "0".into(),
    ]);
    let output = run_os_cli(&valid, Sinks::BrokenStdout)?;
    check(
        output.status.code() == Some(2)
            && output.stdout.is_empty()
            && output.stderr == b"error=diagnostic_output_failed\n",
        "actual broken stdout pipe lost diagnostic error/exit",
    )?;
    let output = run_cli(&[], Sinks::BrokenStderr)?;
    check(
        output.status.code() == Some(1) && output.stdout.is_empty() && output.stderr.is_empty(),
        "actual broken stderr pipe lost failure exit",
    )?;
    Ok(())
}
