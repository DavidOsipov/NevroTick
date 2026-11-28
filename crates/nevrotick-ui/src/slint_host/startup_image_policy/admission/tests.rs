// SPDX-License-Identifier: GPL-3.0-only
//! Independent guard order, first-refusal and backend invocation observations.

use std::cell::{Cell, RefCell};

use super::super::{
    DllSearchPolicyError, ImageLoadPolicyError, RestrictedExecutionPolicy,
    RestrictedExecutionPolicyError, StrictHandlePolicyError,
};
use super::{StartupAdmissionError, StartupCalls, before_backend_with};

/// One guard or backend attempt observed at the shared startup transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Call {
    /// Admit image-load restrictions.
    ImageLoad,
    /// Configure the process default DLL search.
    DllSearch,
    /// Admit permanent strict handles.
    StrictHandles,
    /// Admit dynamic-code restrictions.
    DynamicCode,
    /// Admit extension-point restrictions.
    ExtensionPoints,
    /// Construct the backend after every guard succeeded.
    Backend,
}

/// Fixed typed outcomes with a shared observation of guard and backend attempts.
struct ScriptedCalls<'events> {
    /// The actual sequence, shared only so the factory can record its own call.
    events: &'events RefCell<Vec<Call>>,
    /// Original image-load outcome.
    image_load: Result<(), ImageLoadPolicyError>,
    /// Original DLL-search outcome.
    dll_search: Result<(), DllSearchPolicyError>,
    /// Original strict-handle outcome.
    strict_handles: Result<(), StrictHandlePolicyError>,
    /// Original dynamic-code outcome.
    dynamic_code: Result<(), RestrictedExecutionPolicyError>,
    /// Original extension-point outcome.
    extension_points: Result<(), RestrictedExecutionPolicyError>,
}

impl<'events> ScriptedCalls<'events> {
    /// Supply an accepted outcome for every required guard.
    fn healthy(events: &'events RefCell<Vec<Call>>) -> Self {
        Self {
            events,
            image_load: Ok(()),
            dll_search: Ok(()),
            strict_handles: Ok(()),
            dynamic_code: Ok(()),
            extension_points: Ok(()),
        }
    }
}

impl StartupCalls for ScriptedCalls<'_> {
    /// Record the image-load attempt and return its fixed typed outcome.
    fn image_load(&mut self) -> Result<(), ImageLoadPolicyError> {
        self.events.borrow_mut().push(Call::ImageLoad);
        self.image_load
    }

    /// Record the DLL-search attempt and return its fixed typed outcome.
    fn dll_search(&mut self) -> Result<(), DllSearchPolicyError> {
        self.events.borrow_mut().push(Call::DllSearch);
        self.dll_search
    }

    /// Record the strict-handle attempt and return its fixed typed outcome.
    fn strict_handles(&mut self) -> Result<(), StrictHandlePolicyError> {
        self.events.borrow_mut().push(Call::StrictHandles);
        self.strict_handles
    }

    /// Record the dynamic-code attempt and return its fixed typed outcome.
    fn dynamic_code(&mut self) -> Result<(), RestrictedExecutionPolicyError> {
        self.events.borrow_mut().push(Call::DynamicCode);
        self.dynamic_code
    }

    /// Record the extension-point attempt and return its fixed typed outcome.
    fn extension_points(&mut self) -> Result<(), RestrictedExecutionPolicyError> {
        self.events.borrow_mut().push(Call::ExtensionPoints);
        self.extension_points
    }
}

/// Every required guard precedes the sole factory call and its owned return value.
///
/// # Panics
/// A guard is skipped or reordered, or the factory is retried or its value lost.
#[test]
fn all_required_guards_precede_one_backend_construction() {
    let events: RefCell<Vec<Call>> = RefCell::new(Vec::new());
    let mut calls: ScriptedCalls<'_> = ScriptedCalls::healthy(&events);
    let backend_calls: Cell<u32> = Cell::new(0);
    let backend: Box<u32> = Box::new(42);
    let outcome: Result<Box<u32>, StartupAdmissionError> = before_backend_with(&mut calls, || {
        events.borrow_mut().push(Call::Backend);
        backend_calls.set(backend_calls.get() + 1);
        backend
    });
    assert_eq!(outcome, Ok(Box::new(42)));
    assert_eq!(backend_calls.get(), 1);
    assert_eq!(
        events.borrow().as_slice(),
        [
            Call::ImageLoad,
            Call::DllSearch,
            Call::StrictHandles,
            Call::DynamicCode,
            Call::ExtensionPoints,
            Call::Backend,
        ]
    );
}

/// Poisoned later stages cannot replace the first refusal or reach the backend.
///
/// # Panics
/// Any stage loses its typed cause, invokes a later guard or reaches the factory.
#[test]
fn each_failed_guard_preserves_the_first_typed_cause_and_stops_startup() {
    let cases: [(StartupAdmissionError, &[Call]); 5] = [
        (
            StartupAdmissionError::ImageLoad(ImageLoadPolicyError::ReadEffective(1001)),
            &[Call::ImageLoad],
        ),
        (
            StartupAdmissionError::DllSearch(DllSearchPolicyError { code: 1002 }),
            &[Call::ImageLoad, Call::DllSearch],
        ),
        (
            StartupAdmissionError::StrictHandle(StrictHandlePolicyError::MissingEffectiveFlags(1)),
            &[Call::ImageLoad, Call::DllSearch, Call::StrictHandles],
        ),
        (
            StartupAdmissionError::RestrictedExecution(
                RestrictedExecutionPolicyError::ReadInitial {
                    policy: RestrictedExecutionPolicy::DynamicCode,
                    code: 1004,
                },
            ),
            &[
                Call::ImageLoad,
                Call::DllSearch,
                Call::StrictHandles,
                Call::DynamicCode,
            ],
        ),
        (
            StartupAdmissionError::RestrictedExecution(
                RestrictedExecutionPolicyError::MissingEffectiveFlags {
                    policy: RestrictedExecutionPolicy::ExtensionPoints,
                    requested: 1,
                    observed: 0,
                },
            ),
            &[
                Call::ImageLoad,
                Call::DllSearch,
                Call::StrictHandles,
                Call::DynamicCode,
                Call::ExtensionPoints,
            ],
        ),
    ];
    for (expected, expected_calls) in cases {
        assert_first_refusal(expected, expected_calls);
    }
}

/// Admit only the healthy prefix and independently observe its first typed refusal.
///
/// Later guards deliberately have different failures, so invoking or replacing
/// them cannot reproduce the expected first cause or the fixed call sequence.
///
/// # Panics
/// The transition loses the expected error, attempts a later call or runs the factory.
fn assert_first_refusal(expected: StartupAdmissionError, expected_calls: &[Call]) {
    let events: RefCell<Vec<Call>> = RefCell::new(Vec::new());
    let mut calls: ScriptedCalls<'_> = ScriptedCalls {
        events: &events,
        image_load: Err(ImageLoadPolicyError::Apply(2001)),
        dll_search: Err(DllSearchPolicyError { code: 2002 }),
        strict_handles: Err(StrictHandlePolicyError::ReadInitial(2003)),
        dynamic_code: Err(RestrictedExecutionPolicyError::Apply {
            policy: RestrictedExecutionPolicy::DynamicCode,
            code: 2004,
        }),
        extension_points: Err(RestrictedExecutionPolicyError::ReadEffective {
            policy: RestrictedExecutionPolicy::ExtensionPoints,
            code: 2005,
        }),
    };
    match expected {
        StartupAdmissionError::ImageLoad(error) => calls.image_load = Err(error),
        StartupAdmissionError::DllSearch(error) => {
            calls.image_load = Ok(());
            calls.dll_search = Err(error);
        }
        StartupAdmissionError::StrictHandle(error) => {
            calls.image_load = Ok(());
            calls.dll_search = Ok(());
            calls.strict_handles = Err(error);
        }
        StartupAdmissionError::RestrictedExecution(error) => {
            calls.image_load = Ok(());
            calls.dll_search = Ok(());
            calls.strict_handles = Ok(());
            if expected_calls.len() == 4 {
                calls.dynamic_code = Err(error);
            } else {
                calls.dynamic_code = Ok(());
                calls.extension_points = Err(error);
            }
        }
    }
    let backend_calls: Cell<u32> = Cell::new(0);
    let outcome: Result<(), StartupAdmissionError> = before_backend_with(&mut calls, || {
        events.borrow_mut().push(Call::Backend);
        backend_calls.set(backend_calls.get() + 1);
    });
    assert_eq!(outcome, Err(expected));
    assert_eq!(events.borrow().as_slice(), expected_calls);
    assert_eq!(backend_calls.get(), 0);
}

/// A factory refusal independent of protection admission, deliberately not Copy.
#[derive(Debug, Eq, PartialEq)]
struct BackendFailure {
    /// Original fixed diagnostic payload, moved through the factory result.
    code: u32,
}

/// A fallible backend result passes through unchanged after one complete admission.
///
/// # Panics
/// The backend failure is flattened, replaced, retried or preceded by partial admission.
#[test]
fn backend_failure_is_returned_unchanged_without_retrying_admission() {
    let events: RefCell<Vec<Call>> = RefCell::new(Vec::new());
    let mut calls: ScriptedCalls<'_> = ScriptedCalls::healthy(&events);
    let backend_calls: Cell<u32> = Cell::new(0);
    let failure: BackendFailure = BackendFailure { code: 314 };
    let outcome: Result<Result<(), BackendFailure>, StartupAdmissionError> =
        before_backend_with(&mut calls, || {
            events.borrow_mut().push(Call::Backend);
            backend_calls.set(backend_calls.get() + 1);
            Err(failure)
        });
    assert_eq!(outcome, Ok(Err(BackendFailure { code: 314 })));
    assert_eq!(backend_calls.get(), 1);
    assert_eq!(
        events.borrow().as_slice(),
        [
            Call::ImageLoad,
            Call::DllSearch,
            Call::StrictHandles,
            Call::DynamicCode,
            Call::ExtensionPoints,
            Call::Backend,
        ]
    );
}
