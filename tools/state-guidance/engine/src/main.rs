// SPDX-License-Identifier: GPL-2.0-only
//! One-worker, finite LibAFL comparison over a shared independent trace oracle.
//! Coverage is real LLVM pcguard coverage of the three allowlisted SUT crates.
//! Semantic observations come from reached product outcomes; they are not the
//! oracle's expected state. Both modes use identical mutation and input bounds.

mod coverage;

use std::{
    collections::BTreeSet,
    env, fs,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

use libafl::{
    Error,
    corpus::{Corpus, InMemoryCorpus},
    events::SimpleEventManager,
    executors::{Executor, ExitKind, HasObservers},
    feedback_or,
    feedbacks::{CrashFeedback, MaxMapFeedback},
    fuzzer::{Evaluator, StdFuzzer},
    inputs::{BytesInput, HasTargetBytes},
    monitors::SimpleMonitor,
    observers::OwnedMapObserver,
    schedulers::QueueScheduler,
    state::{HasCorpus, HasExecutions, StdState},
};
use libafl_bolts::{
    ToSlice,
    rands::StdRand,
    tuples::{RefIndexable, tuple_list},
};
use serde_json::{Value, json};
use state_guidance_harness::{
    EffectTraceFailure, EffectTraceObservation, MAX_STEPS, MAX_TRACE_BYTES, SEMANTIC_FEATURE_COUNT,
    TRACE_VERSION, TraceFailure, TraceObservation, execute_effect_trace, execute_trace,
};

type Observers = (OwnedMapObserver<u8>, (OwnedMapObserver<u8>, ()));
const MAX_EXECUTIONS: usize = 65_536;
const MAX_SEEDS: usize = 256;

/// Oracle selection is explicit; existing calls retain exact contract checking.
#[derive(Clone, Copy)]
enum Oracle {
    Strict,
    Effects,
}

impl Oracle {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "strict" => Ok(Self::Strict),
            "effects" => Ok(Self::Effects),
            _ => Err("oracle must be strict or effects".into()),
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Effects => "effects",
        }
    }
}

/// Preserve each oracle's typed result; observations always come from the SUT.
enum ObservedResult {
    Strict(Result<Box<TraceObservation>, TraceFailure>),
    Effects(Result<Box<EffectTraceObservation>, EffectTraceFailure>),
}

impl ObservedResult {
    fn run(bytes: &[u8], oracle: Oracle) -> Self {
        match oracle {
            Oracle::Strict => Self::Strict(execute_trace(bytes).map(Box::new)),
            Oracle::Effects => Self::Effects(execute_effect_trace(bytes).map(Box::new)),
        }
    }

    fn observation(&self) -> Option<&TraceObservation> {
        match self {
            Self::Strict(Ok(value)) => Some(value),
            Self::Effects(Ok(value)) => Some(value.observation()),
            _ => None,
        }
    }

    fn cause(&self) -> Option<TraceFailure> {
        match self {
            Self::Strict(Err(value)) => Some(*value),
            Self::Effects(Err(value)) => Some(value.cause()),
            _ => None,
        }
    }

    fn effect_kind(&self) -> Option<String> {
        match self {
            Self::Effects(Err(value)) => Some(format!("{:?}", value.kind())),
            _ => None,
        }
    }

    fn differences(&self) -> Value {
        let observation = match self {
            Self::Effects(Ok(value)) => Some(value.as_ref()),
            Self::Effects(Err(value)) => Some(value.partial_observation()),
            _ => None,
        };
        json!(observation.map_or_else(Vec::new, |value| {
            value
                .outcome_differences()
                .iter()
                .map(|item| {
                    json!({
                        "step": item.step(),
                        "expected": format!("{:?}", item.expected()),
                        "actual": format!("{:?}", item.actual()),
                    })
                })
                .collect::<Vec<_>>()
        }))
    }

    fn difference_count(&self) -> usize {
        match self {
            Self::Effects(Ok(value)) => value.outcome_difference_count(),
            Self::Effects(Err(value)) => value.partial_observation().outcome_difference_count(),
            _ => 0,
        }
    }
}

/// A concrete executor owns both observer maps, so the driver never aliases
/// borrowed maps or exposes LLVM runtime pointers to LibAFL.
struct TraceExecutor {
    observers: Observers,
    semantic_enabled: bool,
    observations: BTreeSet<usize>,
    last_semantic: BTreeSet<usize>,
    edges: BTreeSet<usize>,
    malformed: usize,
    oracle_failures: usize,
    total_steps: usize,
    completions: usize,
    last_failure: Option<String>,
    current_input: fs::File,
    oracle: Oracle,
    effect_failures: usize,
    outcome_differences: usize,
    last_outcome_differences: Value,
    last_effect_failure: Option<String>,
}

impl TraceExecutor {
    /// Fixed-size maps and empty campaign statistics; input reset is per trace.
    fn new(observers: Observers, semantic_enabled: bool, current_input: fs::File) -> Self {
        Self::with_oracle(observers, semantic_enabled, current_input, Oracle::Strict)
    }

    fn with_oracle(
        observers: Observers,
        semantic_enabled: bool,
        current_input: fs::File,
        oracle: Oracle,
    ) -> Self {
        Self {
            observers,
            semantic_enabled,
            observations: BTreeSet::new(),
            last_semantic: BTreeSet::new(),
            edges: BTreeSet::new(),
            malformed: 0,
            oracle_failures: 0,
            total_steps: 0,
            completions: 0,
            last_failure: None,
            current_input,
            oracle,
            effect_failures: 0,
            outcome_differences: 0,
            last_outcome_differences: json!([]),
            last_effect_failure: None,
        }
    }
}

impl HasObservers for TraceExecutor {
    type Observers = Observers;

    fn observers(&self) -> RefIndexable<&Observers, Observers> {
        RefIndexable::from(&self.observers)
    }

    fn observers_mut(&mut self) -> RefIndexable<&mut Observers, Observers> {
        RefIndexable::from(&mut self.observers)
    }
}

impl<EM, S: HasExecutions, Z> Executor<EM, BytesInput, S, Z> for TraceExecutor {
    /// Exactly one real trace per call. A model/setup failure uses LibAFL's
    /// objective channel but is reported as such, never as sanitizer evidence.
    fn run_target(
        &mut self,
        _fuzzer: &mut Z,
        state: &mut S,
        _manager: &mut EM,
        input: &BytesInput,
    ) -> Result<ExitKind, Error> {
        let bytes = input.target_bytes();
        if bytes.to_slice().len() > MAX_TRACE_BYTES {
            return Err(Error::illegal_argument(
                "trace exceeds campaign input bound",
            ));
        }
        // This handle was created with create_new in the owned campaign output.
        // Reuse the same inode: do not reopen or follow a replaceable path. A
        // completed write precedes target entry, retaining exact bytes even if
        // the target aborts before a result or first-failure report can exist.
        // This is process-crash preservation, not power-loss durability.
        self.current_input
            .set_len(0)
            .and_then(|()| self.current_input.rewind())
            .and_then(|()| self.current_input.write_all(bytes.to_slice()))
            .and_then(|()| self.current_input.flush())
            .map_err(|error| {
                Error::illegal_state(format!("current-input witness write failed: {error}"))
            })?;
        *state.executions_mut() += 1;
        coverage::begin();
        let result = ObservedResult::run(bytes.to_slice(), self.oracle);
        coverage::end();
        coverage::copy_into(&mut self.observers.0)
            .map_err(|message| Error::illegal_state(message.to_owned()))?;
        self.edges.extend(
            self.observers
                .0
                .iter()
                .enumerate()
                .filter_map(|(index, value)| (*value != 0).then_some(index)),
        );
        // Both modes carry the same observation machinery. Only corpus-admission
        // feedback changes: baseline's semantic observer remains all zero.
        self.observers.1.0.fill(0);
        self.last_semantic.clear();
        self.outcome_differences += result.difference_count();
        self.last_outcome_differences = result.differences();
        if let Some(observation) = result.observation() {
            if self.observers.0.iter().all(|value| *value == 0) {
                return Err(Error::illegal_state(
                    "valid trace reached no real SUT LLVM edges",
                ));
            }
            self.total_steps += observation.step_count();
            self.completions += observation.completions();
            for (index, value) in observation.semantic_features().iter().enumerate() {
                if *value != 0 {
                    self.observations.insert(index);
                    self.last_semantic.insert(index);
                }
            }
            if self.semantic_enabled {
                self.observers
                    .1
                    .0
                    .copy_from_slice(observation.semantic_features());
            }
            return Ok(ExitKind::Ok);
        }
        match result.cause() {
            Some(failure) if failure.is_malformed() => {
                self.malformed += 1;
                Ok(ExitKind::Ok)
            }
            Some(TraceFailure::Setup) => Err(Error::illegal_state("fixed fixture setup failed")),
            Some(failure) => {
                self.oracle_failures += 1;
                self.last_failure = Some(format!("{failure:?}"));
                self.last_effect_failure = result.effect_kind();
                self.effect_failures += usize::from(self.last_effect_failure.is_some());
                Ok(ExitKind::Crash)
            }
            None => Err(Error::illegal_state(
                "oracle returned neither observation nor failure",
            )),
        }
    }
}

/// A fixed mutation recipe with explicit seed semantics. This is a small finite
/// pilot, not a claim to reproduce AFL++'s complete mutation pipeline.
struct MutationRng(u64);

impl MutationRng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    /// Every caller supplies a positive small bounded domain.
    fn index(&mut self, length: usize) -> usize {
        (self.next() % length as u64) as usize
    }
}

/// Preserve the versioned, bounded command framing while changing real actions.
fn mutate(parent: &[u8], random: &mut MutationRng) -> Vec<u8> {
    let mut trace = parent.to_vec();
    let steps = (trace.len() - 1) / 2;
    match random.index(4) {
        0 => {
            let index = 1 + 2 * random.index(steps);
            trace[index] = random.index(16) as u8;
        }
        1 => {
            let index = 2 + 2 * random.index(steps);
            trace[index] = random.next() as u8;
        }
        2 if steps < MAX_STEPS => {
            let index = 1 + 2 * random.index(steps + 1);
            trace.splice(index..index, [random.index(16) as u8, random.next() as u8]);
        }
        3 if steps > 1 => {
            let index = 1 + 2 * random.index(steps);
            trace.drain(index..index + 2);
        }
        _ => {
            let index = 2 + 2 * random.index(steps);
            trace[index] ^= 1 << random.index(8);
        }
    }
    trace
}

/// One experimental operator preserves a prefix and appends a normal request
/// succession. Parameters and prefix boundary vary without consulting the oracle.
fn lifecycle_suffix(parent: &[u8], random: &mut MutationRng) -> Vec<u8> {
    const SUFFIX_STEPS: usize = 6;
    let steps = (parent.len() - 1) / 2;
    let prefix_steps = random.index(steps.min(MAX_STEPS - SUFFIX_STEPS) + 1);
    let mut result = parent[..1 + 2 * prefix_steps].to_vec();
    let first = random.next() as u8 | 0x80;
    let second = random.next() as u8 | 0x80;
    // Retire old work, complete one request, then challenge a successor with an
    // obsolete identity before its valid response. Long deadlines keep this
    // operator's normal flow independent of accidental short timing windows.
    result.extend_from_slice(&[4, 0, 0, first, 1, 0, 0, second, 5, 0, 1, 0]);
    result
}

#[derive(Clone, Copy)]
enum MutationStrategy {
    Baseline,
    Lifecycle,
}

impl MutationStrategy {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "baseline" => Ok(Self::Baseline),
            "lifecycle" => Ok(Self::Lifecycle),
            _ => Err("mutation must be baseline or lifecycle".into()),
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Lifecycle => "lifecycle",
        }
    }

    const fn recipe(self) -> &'static str {
        match self {
            Self::Baseline => "bounded-command-mutations-v1",
            Self::Lifecycle => "bounded-lifecycle-suffix-v1",
        }
    }

    /// Baseline consumes exactly the earlier RNG stream. The experimental arm
    /// selects the one suffix operator with probability 1/4, otherwise baseline.
    fn apply(self, parent: &[u8], random: &mut MutationRng) -> (Vec<u8>, bool) {
        if matches!(self, Self::Lifecycle) && random.index(4) == 0 {
            (lifecycle_suffix(parent, random), true)
        } else {
            (mutate(parent, random), false)
        }
    }
}

/// Bound bytes before allocation; source paths must be canonical regular files.
fn read_trace(path: &Path) -> Result<Vec<u8>, String> {
    if !path.is_absolute() || path.canonicalize().map_err(|e| e.to_string())? != path {
        return Err("input must be an absolute canonical path".into());
    }
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.len() > (MAX_TRACE_BYTES + 1) as u64 {
        return Err("input must be a bounded regular trace file".into());
    }
    let mut bytes = Vec::with_capacity(MAX_TRACE_BYTES + 1);
    fs::File::open(path)
        .map_err(|e| e.to_string())?
        .take((MAX_TRACE_BYTES + 2) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_TRACE_BYTES + 1 {
        return Err("input exceeded the bounded replay-read budget".into());
    }
    Ok(bytes)
}

/// Seed admission preserves the mutation recipe's nonempty command preconditions.
/// Replay deliberately forwards malformed raw bytes to the real driver instead.
fn admit_seed(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 3
        || bytes.len() > MAX_TRACE_BYTES
        || bytes.len() % 2 != 1
        || bytes[0] != TRACE_VERSION
        || bytes[1..]
            .as_chunks::<2>()
            .0
            .iter()
            .any(|command| command[0] >= 16)
    {
        return Err("input has unsupported command framing".into());
    }
    Ok(())
}

/// Write one new file only, preserving existing evidence on every error path.
fn write_new(path: &Path, contents: &[u8]) -> Result<(), String> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(contents).map_err(|e| e.to_string())
}

fn indices(map: &[u8]) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter_map(|(index, value)| (*value != 0).then_some(index))
        .collect()
}

/// One standalone replay supplies concrete reached observations, not a seed-only
/// reconstruction whose meaning could change with a generator revision.
fn replay(path: &Path, oracle: Oracle) -> Result<Value, String> {
    let bytes = read_trace(path)?;
    coverage::begin();
    let result = ObservedResult::run(&bytes, oracle);
    coverage::end();
    let mut map = vec![0; coverage::MAP_CAPACITY];
    coverage::copy_into(&mut map).map_err(str::to_owned)?;
    if let Some(observation) = result.observation() {
        return Ok(json!({
            "schema_version": 1,
            "status": "ok",
            "steps": observation.step_count(),
            "completions": observation.completions(),
            "final_pending": observation.final_pending(),
            "semantic_features": indices(observation.semantic_features()),
            "coverage_edges": indices(&map),
            "registered_guards": coverage::registered_guards(),
            "oracle": oracle.name(),
            "outcome_differences": result.differences(),
        }));
    }
    match result.cause() {
        Some(TraceFailure::Setup) => Err("fixed fixture setup failed".into()),
        Some(failure) => Ok(json!({
            "schema_version": 1,
            "status": if failure.is_malformed() {"invalid_input"} else {"oracle_failure"},
            "steps": if failure.is_malformed() {Some(0)} else {None::<usize>},
            "failure": format!("{failure:?}"),
            "coverage_edges": indices(&map),
            "registered_guards": coverage::registered_guards(),
            "oracle": oracle.name(),
            "effect_failure_kind": result.effect_kind(),
            "outcome_differences": result.differences(),
        })),
        None => Err("oracle returned neither observation nor failure".into()),
    }
}

/// Run exactly the requested number of evaluate_input calls, counting supplied
/// seeds inside that budget. Each call invokes the same target/model once.
fn campaign(
    mode: &str,
    seed: u64,
    budget: usize,
    seeds_path: &Path,
    output: &Path,
    strategy: MutationStrategy,
    oracle: Oracle,
) -> Result<Value, String> {
    if !matches!(mode, "coverage" | "semantic") || !(1..=MAX_EXECUTIONS).contains(&budget) {
        return Err("mode or execution budget is unsupported".into());
    }
    if !seeds_path.is_absolute()
        || seeds_path.canonicalize().map_err(|e| e.to_string())? != seeds_path
        || !seeds_path.is_dir()
    {
        return Err("seed directory must be absolute and canonical".into());
    }
    let mut paths = fs::read_dir(seeds_path)
        .map_err(|e| e.to_string())?
        .map(|entry| entry.map(|entry| entry.path()))
        .take(MAX_SEEDS + 1)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    paths.sort();
    if paths.is_empty() || paths.len() > MAX_SEEDS || paths.len() > budget {
        return Err("seed count is empty or exceeds the declared budget".into());
    }
    let seeds = paths
        .iter()
        .map(|path| read_trace(path))
        .collect::<Result<Vec<_>, _>>()?;
    for bytes in &seeds {
        admit_seed(bytes)?;
    }
    if !output.is_absolute()
        || output
            .parent()
            .ok_or("output has no parent")?
            .canonicalize()
            .map_err(|e| e.to_string())?
            != output.parent().ok_or("output has no parent")?
    {
        return Err("output must have an absolute canonical parent".into());
    }
    fs::create_dir(output).map_err(|e| e.to_string())?;
    let queue = output.join("queue");
    fs::create_dir(&queue).map_err(|e| e.to_string())?;
    let coverage_observer =
        OwnedMapObserver::new("sut-llvm-edges", vec![0_u8; coverage::MAP_CAPACITY]);
    let semantic_observer =
        OwnedMapObserver::new("actual-semantic-states", vec![0_u8; SEMANTIC_FEATURE_COUNT]);
    let mut feedback = feedback_or!(
        MaxMapFeedback::new(&coverage_observer),
        MaxMapFeedback::new(&semantic_observer)
    );
    let mut objective = CrashFeedback::new();
    let mut state = StdState::new(
        StdRand::with_seed(seed),
        InMemoryCorpus::<BytesInput>::new(),
        InMemoryCorpus::<BytesInput>::new(),
        &mut feedback,
        &mut objective,
    )
    .map_err(|e| e.to_string())?;
    let mut manager = SimpleEventManager::new(SimpleMonitor::new(|_| {}));
    let mut fuzzer = StdFuzzer::new(QueueScheduler::new(), feedback, objective);
    let current_input = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join("current-input.trace"))
        .map_err(|e| e.to_string())?;
    let observers = tuple_list!(coverage_observer, semantic_observer);
    let mut executor = match oracle {
        Oracle::Strict => TraceExecutor::new(observers, mode == "semantic", current_input),
        Oracle::Effects => {
            TraceExecutor::with_oracle(observers, mode == "semantic", current_input, oracle)
        }
    };
    let mut random = MutationRng(seed);
    let mut retained = BTreeSet::new();
    let mut queue_files = 0_usize;
    let mut last_failure_input = None;
    let mut lifecycle_operator_calls = 0;
    let mut first_difference_input = None;
    let mut first_difference_execution = None;
    for iteration in 0..budget {
        let bytes = if let Some(seed_input) = seeds.get(iteration) {
            seed_input.clone()
        } else {
            let ids = state.corpus().ids().collect::<Vec<_>>();
            if ids.is_empty() {
                return Err("no seed entered the real coverage corpus".into());
            }
            let testcase = state
                .corpus()
                .get(ids[random.index(ids.len())])
                .map_err(|e| e.to_string())?
                .borrow();
            let parent = testcase.input().as_ref().ok_or("corpus input is missing")?;
            let (changed, used_lifecycle) =
                strategy.apply(parent.target_bytes().to_slice(), &mut random);
            lifecycle_operator_calls += usize::from(used_lifecycle);
            changed
        };
        let input = BytesInput::new(bytes.clone());
        let (_, admitted) = fuzzer
            .evaluate_input(&mut state, &mut executor, &mut manager, &input)
            .map_err(|e| e.to_string())?;
        // Preserve one bounded diagnostic witness independently of coverage
        // admission or an eventual fatal failure. Error-order differences may
        // remain nonfatal in the effects lane, but must stay reproducible.
        if first_difference_input.is_none() && executor.last_outcome_differences != json!([]) {
            write_new(&output.join("first-difference.trace"), &bytes)?;
            write_new(
                &output.join("first-difference.json"),
                &serde_json::to_vec_pretty(&executor.last_outcome_differences)
                    .map_err(|e| e.to_string())?,
            )?;
            first_difference_input = Some("first-difference.trace");
            first_difference_execution = Some(*state.executions());
        }
        if executor.oracle_failures != 0 {
            write_new(&output.join("first-failure.trace"), &bytes)?;
            last_failure_input = Some("first-failure.trace");
            break;
        }
        if admitted.is_some() {
            write_new(&queue.join(format!("{queue_files:06}.trace")), &bytes)?;
            queue_files += 1;
            // Replay only the observation already returned by this execution;
            // no extra target callbacks are hidden outside the finite budget.
            retained.extend(executor.last_semantic.iter().copied());
        }
    }
    let report = json!({
        "schema_version": 1,
        "status": if executor.oracle_failures == 0 {"complete"} else {"oracle_failure"},
        "mode": mode,
        "engine": "LibAFL 0.16.1",
        "mutation_recipe": strategy.recipe(),
        "mutation_strategy": strategy.name(),
        "lifecycle_operator_calls": lifecycle_operator_calls,
        "oracle": oracle.name(),
        "effect_failures": executor.effect_failures,
        "outcome_differences": executor.outcome_differences,
        "first_difference_input": first_difference_input,
        "first_difference_execution": first_difference_execution,
        "first_difference_details": first_difference_input.map(|_| "first-difference.json"),
        "first_effect_failure_kind": executor.last_effect_failure,
        "seed": seed,
        "executions_requested": budget,
        "executions_completed": state.executions(),
        "seed_inputs": seeds.len(),
        "malformed_inputs": executor.malformed,
        "oracle_failures": executor.oracle_failures,
        "first_failure": executor.last_failure,
        "first_failure_input": last_failure_input,
        "current_input_trace": "current-input.trace",
        "current_input_contract": "complete bounded bytes written through owned file handle before each target call; process-crash scope",
        "completed_trace_steps": executor.total_steps,
        "completions": executor.completions,
        "corpus_count": state.corpus().count(),
        "saved_queue_files": queue_files,
        "observed_semantic_features": executor.observations,
        "retained_semantic_features": retained,
        "observed_coverage_edges": executor.edges,
        "registered_guards": coverage::registered_guards(),
        "limits": {"max_steps":MAX_STEPS, "max_trace_bytes":MAX_TRACE_BYTES, "single_worker":true},
    });
    write_new(
        &output.join("report.json"),
        &serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )?;
    Ok(report)
}

fn execute() -> Result<Value, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, path] if command == "replay" => replay(&PathBuf::from(path), Oracle::Strict),
        [command, path, oracle] if command == "replay" => replay(&PathBuf::from(path), Oracle::parse(oracle)?),
        [command, mode, seed, budget, seeds, output] if command == "campaign" => campaign(
            mode,
            seed.parse().map_err(|_| "seed must be u64")?,
            budget.parse().map_err(|_| "budget must be an unsigned integer")?,
            &PathBuf::from(seeds),
            &PathBuf::from(output),
            MutationStrategy::Baseline,
            Oracle::Strict,
        ),
        [command, mode, seed, budget, seeds, output, mutation, oracle] if command == "campaign" => campaign(
            mode,
            seed.parse().map_err(|_| "seed must be u64")?,
            budget.parse().map_err(|_| "budget must be an unsigned integer")?,
            &PathBuf::from(seeds),
            &PathBuf::from(output),
            MutationStrategy::parse(mutation)?,
            Oracle::parse(oracle)?,
        ),
        _ => Err("usage: state-guidance replay ABS_TRACE [strict|effects] | campaign coverage|semantic SEED EXECUTIONS ABS_SEEDS ABS_NEW_OUTPUT [baseline|lifecycle strict|effects]".into()),
    }
}

fn main() {
    match execute() {
        Ok(report) => {
            let failed = !matches!(
                report["status"].as_str(),
                Some("ok" | "complete" | "invalid_input")
            );
            println!("{report}");
            if failed {
                std::process::exit(86);
            }
        }
        Err(message) => {
            eprintln!("state-guidance incomplete: {message}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Instrumented SUT calls share the process-wide one-worker coverage map.
    // Keep their ownership exclusive even when libtest schedules other tests.
    static SUT_CUSTODY: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn baseline_strategy_preserves_the_previous_rng_and_mutation_stream() {
        let parent = [TRACE_VERSION, 0, 0xf0, 1, 0];
        let mut old = MutationRng(23);
        let mut baseline = MutationRng(23);
        for _ in 0..128 {
            let expected = mutate(&parent, &mut old);
            let (actual, used_lifecycle) = MutationStrategy::Baseline.apply(&parent, &mut baseline);
            assert_eq!(actual, expected);
            assert!(!used_lifecycle);
        }
    }

    #[test]
    fn lifecycle_operator_preserves_a_prefix_and_obeys_maximum_trace_bounds() {
        let _custody = SUT_CUSTODY.lock().expect("exclusive instrumented SUT test");
        let mut parent = vec![TRACE_VERSION];
        for _ in 0..MAX_STEPS {
            parent.extend_from_slice(&[0, 0xf0]);
        }
        let mut random = MutationRng(37);
        for _ in 0..128 {
            let changed = lifecycle_suffix(&parent, &mut random);
            assert!((13..=MAX_TRACE_BYTES).contains(&changed.len()));
            let prefix_len = changed.len() - 12;
            assert_eq!(&changed[..prefix_len], &parent[..prefix_len]);
            let suffix = &changed[prefix_len..];
            assert_eq!(
                [
                    suffix[0], suffix[2], suffix[4], suffix[6], suffix[8], suffix[10]
                ],
                [4, 0, 1, 0, 5, 1]
            );
            assert!(suffix[3] >= 0x80 && suffix[7] >= 0x80);
            assert!(execute_trace(&changed).is_ok());
        }
    }

    #[test]
    fn lifecycle_strategy_is_repeatable_and_reaches_both_operator_paths() {
        let parent = [TRACE_VERSION, 0, 0xf0];
        let mut first = MutationRng(91);
        let mut second = MutationRng(91);
        let mut counts = [0_usize; 2];
        for _ in 0..256 {
            let (left, selected) = MutationStrategy::Lifecycle.apply(&parent, &mut first);
            assert_eq!(
                (left.clone(), selected),
                MutationStrategy::Lifecycle.apply(&parent, &mut second)
            );
            admit_seed(&left).expect("operator preserves admitted command framing");
            counts[usize::from(selected)] += 1;
        }
        assert!(counts.iter().all(|value| *value > 0));
        assert!(MutationStrategy::parse("unknown").is_err());
        assert!(Oracle::parse("unknown").is_err());
    }

    #[test]
    fn mutations_preserve_the_declared_trace_domain() {
        let mut random = MutationRng(7);
        let mut trace = vec![TRACE_VERSION, 0, 0];
        for _ in 0..4096 {
            trace = mutate(&trace, &mut random);
            assert!((3..=MAX_TRACE_BYTES).contains(&trace.len()));
            assert_eq!(trace.len() % 2, 1);
            assert_eq!(trace[0], TRACE_VERSION);
            assert!(
                trace[1..]
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .all(|command| command[0] < 16)
            );
        }
    }

    #[test]
    fn mutation_seed_is_repeatable_and_changes_the_actual_trace() {
        let parent = [TRACE_VERSION, 0, 0];
        let mut first = MutationRng(17);
        let mut second = MutationRng(17);
        let left = (0..16)
            .map(|_| mutate(&parent, &mut first))
            .collect::<Vec<_>>();
        let right = (0..16)
            .map(|_| mutate(&parent, &mut second))
            .collect::<Vec<_>>();
        assert_eq!(left, right);
        assert!(left.iter().any(|trace| trace != &parent));
    }

    #[test]
    fn seed_admission_retains_valid_neighbor_and_rejects_unsupported_framing() {
        assert!(admit_seed(&[TRACE_VERSION, 0, 0]).is_ok());
        for invalid in [
            vec![],
            vec![TRACE_VERSION],
            vec![0, 0, 0],
            vec![TRACE_VERSION, 0],
            vec![TRACE_VERSION, 16, 0],
            vec![TRACE_VERSION; MAX_TRACE_BYTES + 1],
        ] {
            assert!(admit_seed(&invalid).is_err());
        }
    }

    #[test]
    fn executor_resets_real_observations_between_a_b_a_in_one_process() {
        let _custody = SUT_CUSTODY.lock().expect("exclusive instrumented SUT test");
        struct Count(u64);
        impl HasExecutions for Count {
            fn executions(&self) -> &u64 {
                &self.0
            }
            fn executions_mut(&mut self) -> &mut u64 {
                &mut self.0
            }
        }
        // The externally owned test runner supplies private TMPDIR. create_new
        // refuses stale/unowned evidence; Drop is fallback after assertions.
        struct Witness(PathBuf);
        impl Drop for Witness {
            fn drop(&mut self) {
                let _ = fs::remove_file(&self.0);
            }
        }
        let witness_path = env::temp_dir().join(format!(
            "nevrotick-observer-reset-{}.trace",
            std::process::id()
        ));
        let witness_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&witness_path)
            .expect("owned fresh witness");
        let witness = Witness(witness_path);
        let mut executor = TraceExecutor::new(
            tuple_list!(
                OwnedMapObserver::new("reset-sut-edges", vec![0_u8; coverage::MAP_CAPACITY]),
                OwnedMapObserver::new("reset-actual-states", vec![0_u8; SEMANTIC_FEATURE_COUNT])
            ),
            true,
            witness_file,
        );
        let mut count = Count(0);
        let mut first_coverage = None;
        for (trace, expected_semantic) in [([1, 8, 0], 275), ([1, 8, 1], 279), ([1, 8, 0], 275)] {
            let input = BytesInput::new(trace.to_vec());
            assert_eq!(
                executor
                    .run_target(&mut (), &mut count, &mut (), &input)
                    .expect("reached valid trace"),
                ExitKind::Ok
            );
            assert_eq!(indices(&executor.observers.1.0), vec![expected_semantic]);
            let observed_coverage = indices(&executor.observers.0);
            assert!(!observed_coverage.is_empty());
            if let Some(first) = &first_coverage {
                assert_eq!(&observed_coverage, first);
            } else {
                first_coverage = Some(observed_coverage);
            }
            assert_eq!(fs::read(&witness.0).expect("retained raw witness"), trace);
        }
        // A/B/A distinguishes semantic-state reset despite identical edge maps.
        // C reaches different product branches; C/A additionally distinguishes
        // real LLVM-map reset from accidentally retaining prior trace coverage.
        let baseline = first_coverage.expect("first trace coverage was observed");
        let different = [1, 0, 240, 1, 0];
        assert_eq!(
            executor
                .run_target(
                    &mut (),
                    &mut count,
                    &mut (),
                    &BytesInput::new(different.to_vec())
                )
                .expect("arm and complete trace"),
            ExitKind::Ok
        );
        assert_ne!(indices(&executor.observers.0), baseline);
        assert_eq!(
            fs::read(&witness.0).expect("retained different trace"),
            different
        );
        let original = [1, 8, 0];
        assert_eq!(
            executor
                .run_target(
                    &mut (),
                    &mut count,
                    &mut (),
                    &BytesInput::new(original.to_vec())
                )
                .expect("fresh original trace"),
            ExitKind::Ok
        );
        assert_eq!(indices(&executor.observers.0), baseline);
        assert_eq!(indices(&executor.observers.1.0), vec![275]);
        assert_eq!(
            fs::read(&witness.0).expect("retained final trace"),
            original
        );
        assert_eq!(*count.executions(), 5);
        assert_eq!(executor.total_steps, 6);
    }
}
