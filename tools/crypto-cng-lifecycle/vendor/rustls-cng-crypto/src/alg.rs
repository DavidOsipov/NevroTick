//! Algorithm provider initialization and cleanup.
use once_cell::sync::OnceCell;
use rustls::Error;
use windows::Win32::Foundation::NTSTATUS;
#[cfg(feature = "tls12")]
use windows::Win32::Security::Cryptography::BCRYPT_TLS1_2_KDF_ALGORITHM;
use windows::Win32::Security::Cryptography::{
    BCRYPT_ECC_CURVE_25519, BCRYPT_ECC_CURVE_NAME, BCRYPT_ECDH_ALGORITHM, BCRYPT_HANDLE,
};
#[cfg(not(feature = "fixture-init-probe"))]
use windows::Win32::Security::Cryptography::{
    BCryptCloseAlgorithmProvider, BCryptOpenAlgorithmProvider, BCryptSetProperty,
};
#[cfg(any(not(feature = "fixture-init-probe"), feature = "tls12"))]
use windows::core::Free;
use windows::{
    Win32::Security::Cryptography::{BCRYPT_ALG_HANDLE, BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS},
    core::PCWSTR,
};

/// Safe diagnostic entry point and actual-native-call observation wrappers.
#[cfg(feature = "fixture-init-probe")]
pub mod probe;

/// A handle that, when dropped, will free all algorithm providers initialized by this crate.
///
/// Where possible this crate aims to use the shared providers described in
/// <https://learn.microsoft.com/en-us/windows/win32/seccng/cng-algorithm-pseudo-handles>.
///
/// This should be created once at the start of the program and dropped at the end.
pub struct ShutdownHandle {}

impl Drop for ShutdownHandle {
    fn drop(&mut self) {
        #[cfg(feature = "fixture-init-probe")]
        probe::mark_shutdown_entry();
        unsafe {
            #[cfg(not(feature = "fixture-init-probe"))]
            ecdh_x25519().free();
            #[cfg(feature = "fixture-init-probe")]
            probe::close_shutdown(ecdh_x25519());
            #[cfg(feature = "tls12")]
            tls12_kdf().free();
        }
    }
}

struct Handle(BCRYPT_ALG_HANDLE);
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

pub(crate) fn ecdh_x25519() -> BCRYPT_ALG_HANDLE {
    #[cfg(feature = "fixture-init-probe")]
    probe::mark_cache_access();
    static ALG_HANDLE: OnceCell<Handle> = OnceCell::new();
    ALG_HANDLE
        .get_or_init(|| {
            Handle(
                load_algorithm(
                    BCRYPT_ECDH_ALGORITHM,
                    BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS::default(),
                    Some((BCRYPT_ECC_CURVE_NAME, BCRYPT_ECC_CURVE_25519)),
                )
                .unwrap(),
            )
        })
        .0
}

#[cfg(feature = "tls12")]
pub(crate) fn tls12_kdf() -> BCRYPT_ALG_HANDLE {
    static ALG_HANDLE: OnceCell<Handle> = OnceCell::new();
    ALG_HANDLE
        .get_or_init(|| {
            Handle(
                load_algorithm(
                    BCRYPT_TLS1_2_KDF_ALGORITHM,
                    BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS::default(),
                    None,
                )
                .unwrap(),
            )
        })
        .0
}

/// Own one successful native open until transfer or one close attempt.
struct AlgorithmOwner {
    /// The handle returned by the successful open, never a pseudo-handle.
    handle: BCRYPT_ALG_HANDLE,
    /// Cleared before transfer or native close, including a failed close.
    armed: bool,
}

impl AlgorithmOwner {
    /// Transfer ownership exactly once to the successful initializer's caller.
    fn into_raw(mut self) -> BCRYPT_ALG_HANDLE {
        self.armed = false;
        #[cfg(feature = "fixture-init-probe")]
        probe::record_cache_transfer(self.handle);
        self.handle
    }

    /// Consume ownership before one close; preserve the actual native status.
    fn close(mut self) -> NTSTATUS {
        self.armed = false;
        close_algorithm(self.handle)
    }
}

impl Drop for AlgorithmOwner {
    /// Attempt one close during unwinding; a native failure is never retried.
    fn drop(&mut self) {
        if self.armed {
            self.armed = false;
            let _status = close_algorithm(self.handle);
        }
    }
}

/// Close only a successful open owned by the private guard, without retry.
fn close_algorithm(handle: BCRYPT_ALG_HANDLE) -> NTSTATUS {
    #[cfg(not(feature = "fixture-init-probe"))]
    // SAFETY: The guard owns this real successful open and disarms before closing.
    unsafe {
        BCryptCloseAlgorithmProvider(handle, 0)
    }
    #[cfg(feature = "fixture-init-probe")]
    // SAFETY: The same ownership obligation applies to the observing wrapper.
    unsafe {
        probe::close_algorithm(handle)
    }
}

/// Load an algorithm provider with specified flags, and optional property.
///
/// # Errors
/// Preserves native open/property failure and any explicit error-cleanup failure.
/// Ownership transfers only after all fallible initialization has succeeded.
fn load_algorithm(
    id: PCWSTR,
    flags: BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS,
    property: Option<(PCWSTR, PCWSTR)>,
) -> Result<BCRYPT_ALG_HANDLE, Error> {
    let mut alg_handle = BCRYPT_ALG_HANDLE::default();
    unsafe {
        #[cfg(not(feature = "fixture-init-probe"))]
        let status = BCryptOpenAlgorithmProvider(&mut alg_handle, id, None, flags);
        #[cfg(feature = "fixture-init-probe")]
        let status = probe::open_algorithm(&mut alg_handle, id, flags);
        status
            .ok()
            .map_err(|e| Error::General(format!("BCryptOpenAlgorithmProvider error: {e}")))?;
        let owner = AlgorithmOwner {
            handle: alg_handle,
            armed: true,
        };
        if let Some((property, value)) = property {
            let bcrypt_handle = BCRYPT_HANDLE(alg_handle.0);
            #[cfg(not(feature = "fixture-init-probe"))]
            let status = BCryptSetProperty(
                bcrypt_handle,
                property,
                &to_null_terminated_le_bytes(value),
                0,
            );
            #[cfg(feature = "fixture-init-probe")]
            let status =
                probe::set_property(bcrypt_handle, property, &to_null_terminated_le_bytes(value));
            if let Err(error) = status.ok() {
                let close_status = owner.close();
                return Err(Error::General(if close_status.is_err() {
                    format!(
                        "BCryptSetProperty error: {error}; primary NTSTATUS: {}; BCryptCloseAlgorithmProvider NTSTATUS: {}",
                        status.0, close_status.0
                    )
                } else {
                    format!("BCryptSetProperty error: {error}")
                }));
            }
        }
        Ok(owner.into_raw())
    }
}

fn to_null_terminated_le_bytes(str: PCWSTR) -> Vec<u8> {
    unsafe {
        str.as_wide()
            .iter()
            .copied()
            .chain(Some(0))
            .flat_map(u16::to_le_bytes)
            .collect()
    }
}
