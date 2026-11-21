// SPDX-License-Identifier: GPL-2.0-only
//! Fixed Windows publisher used by the offline service writer fixture.

use std::{
    num::{NonZeroI64, NonZeroU32},
    ptr,
};

use windows_sys::{
    Win32::System::Diagnostics::Etw::{
        EVENT_DATA_DESCRIPTOR, EVENT_DATA_DESCRIPTOR_0, EVENT_DESCRIPTOR, EventRegister,
        EventUnregister, EventWrite, REGHANDLE,
    },
    core::GUID,
};

use crate::diagnostics::{
    ReleaseUnconfirmed, WriterClose,
    threaded::{
        BackendClose, EventDescriptor, RegisterOnce, RegistrationAttempt, SubmitOnce, WriteAttempt,
        WriterBackend,
    },
};

/// Provider of the admitted offline manifest fixture.
static PROVIDER: GUID = GUID::from_u128(0x407b0d7f_0220_4ca1_9e6b_d0a6c0d33380);

/// Dormant publisher with one registration on the selected writer thread.
pub(crate) struct FixturePublisher {
    /// A nonzero SDK registration token, held through actual unregistration.
    registration: Option<NonZeroI64>,
}

impl FixturePublisher {
    /// Prepare an inert publisher before entering the portable service owner.
    pub(crate) const fn new() -> Self {
        Self { registration: None }
    }
}

/// Project an address and extent while the caller retains the original slice.
fn data_descriptor(field: &[u8]) -> Option<EVENT_DATA_DESCRIPTOR> {
    Some(EVENT_DATA_DESCRIPTOR {
        Ptr: u64::try_from(field.as_ptr().expose_provenance()).ok()?,
        Size: u32::try_from(field.len()).ok()?,
        Anonymous: EVENT_DATA_DESCRIPTOR_0 { Reserved: 0 },
    })
}

/// Preserve the five independent field extents in manifest order.
fn payload(fields: [&[u8]; 5]) -> Option<[EVENT_DATA_DESCRIPTOR; 5]> {
    let [component, sequence, kind, elapsed, cause]: [&[u8]; 5] = fields;
    Some([
        data_descriptor(component)?,
        data_descriptor(sequence)?,
        data_descriptor(kind)?,
        data_descriptor(elapsed)?,
        data_descriptor(cause)?,
    ])
}

impl WriterBackend for FixturePublisher {
    /// Claim one actual registration. Keep any returned nonzero token for cleanup.
    fn register(&mut self, registration: RegisterOnce<'_>) -> RegistrationAttempt {
        let mut handle: REGHANDLE = 0;
        registration.register(|| {
            #[expect(
                unsafe_code,
                reason = "one callback-free registration of the fixed fixture provider"
            )]
            // SAFETY: The provider is fixed and the valid out-pointer is live for
            // this call. Both callback and context are null. The writer retains
            // any returned nonzero registration for explicit release.
            let status: u32 =
                unsafe { EventRegister(&raw const PROVIDER, None, ptr::null(), &raw mut handle) };
            self.registration = NonZeroI64::new(handle);
            status == 0 && self.registration.is_some()
        })
    }

    /// Build ABI descriptors before taking the current claim for one actual call.
    fn write(&mut self, submission: SubmitOnce<'_>) -> WriteAttempt {
        let Some(handle): Option<NonZeroI64> = self.registration else {
            return submission.submit(|| false);
        };
        let declared: &EventDescriptor = submission.event().descriptor();
        let descriptor: EVENT_DESCRIPTOR = EVENT_DESCRIPTOR {
            Id: declared.id,
            Version: declared.version,
            Channel: declared.channel,
            Level: declared.level,
            Opcode: declared.opcode,
            Task: declared.task,
            Keyword: declared.keyword,
        };
        let Some(data): Option<[EVENT_DATA_DESCRIPTOR; 5]> = payload(submission.event().fields())
        else {
            return submission.submit(|| false);
        };
        submission.submit(|| {
            #[expect(
                unsafe_code,
                reason = "one synchronous ETW write under the consumed writer claim"
            )]
            // SAFETY: The live registration belongs to this writer. The fixed
            // descriptors and prepared packet remain live and unmoved until
            // return. The five data pointers address exact immutable extents.
            let status: u32 =
                unsafe { EventWrite(handle.get(), &raw const descriptor, 5, data.as_ptr()) };
            status == 0
        })
    }

    /// Release this writer's original registration once on its owned thread.
    ///
    /// # Errors
    /// Preserve the exact failed unregistration and opaque handle bits.
    fn close(self) -> BackendClose {
        let Some(handle): Option<NonZeroI64> = self.registration else {
            return Ok(WriterClose::Drained);
        };
        #[expect(
            unsafe_code,
            reason = "release only the original writer-owned registration"
        )]
        // SAFETY: The registration was assigned to this sole publisher. All
        // synchronous writes returned, and there is no callback or other user.
        let status: u32 = unsafe { EventUnregister(handle.get()) };
        if let Some(status) = NonZeroU32::new(status) {
            return Err(ReleaseUnconfirmed::Provider {
                registration: handle.cast_unsigned(),
                status,
            });
        }
        Ok(WriterClose::Drained)
    }
}
