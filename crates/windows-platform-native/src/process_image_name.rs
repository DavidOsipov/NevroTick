// SPDX-License-Identifier: GPL-2.0-only
//! Bounded readback of the Win32 image name reported for a retained process.

use core::fmt;

/// Fixture capacity in wide units, including room for the required NUL.
pub(super) const PROCESS_IMAGE_NAME_BUFFER: usize = 1_024;

/// Failure to observe a retained process's reported image name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessImageNameError {
    /// The query failed with this immediate Win32 error.
    Native(u32),
    /// A successful query reported an invalid length or NUL placement.
    Malformed,
}

impl fmt::Display for ProcessImageNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native(code) => write!(formatter, "QueryFullProcessImageNameW failed ({code})"),
            Self::Malformed => formatter.write_str("invalid reported process image name"),
        }
    }
}

impl core::error::Error for ProcessImageNameError {}

/// The literal Win32 image name reported for the queried process.
///
/// The receiving owner must establish peer identity and liveness separately
/// and compare the observation with its independently supplied expected path.
#[must_use = "inspect the reported image name before using it as peer evidence"]
pub struct ReportedProcessImageName {
    /// Initialized storage containing the native output.
    units: [u16; PROCESS_IMAGE_NAME_BUFFER],
    /// Checked number of path units before the required NUL.
    length: usize,
}

impl ReportedProcessImageName {
    /// Borrow the exact wide code units, excluding the terminating NUL.
    #[must_use]
    pub fn as_wide(&self) -> &[u16] {
        &self.units[..self.length]
    }
}

/// Interpret one native query whose successful count excludes the NUL.
///
/// Nonzero fill makes a missing terminator observable. Payload units rely
/// on the native success/count contract and are treated as opaque wide data.
///
/// # Errors
/// Preserve a native failure or reject malformed successful output.
pub(super) fn read_process_image_name(
    query: impl FnOnce(&mut [u16; PROCESS_IMAGE_NAME_BUFFER], &mut u32) -> Result<(), u32>,
) -> Result<ReportedProcessImageName, ProcessImageNameError> {
    let mut units: [u16; PROCESS_IMAGE_NAME_BUFFER] = [u16::MAX; PROCESS_IMAGE_NAME_BUFFER];
    let mut count: u32 =
        u32::try_from(units.len()).map_err(|_| ProcessImageNameError::Malformed)?;
    query(&mut units, &mut count).map_err(ProcessImageNameError::Native)?;

    let length: usize = usize::try_from(count).map_err(|_| ProcessImageNameError::Malformed)?;
    let Some((name, suffix)) = units.split_at_checked(length) else {
        return Err(ProcessImageNameError::Malformed);
    };
    if name.is_empty() || suffix.first() != Some(&0) || name.contains(&0) {
        return Err(ProcessImageNameError::Malformed);
    }
    Ok(ReportedProcessImageName { units, length })
}

#[cfg(test)]
mod tests {
    use super::{
        PROCESS_IMAGE_NAME_BUFFER, ProcessImageNameError, ReportedProcessImageName,
        read_process_image_name,
    };

    /// Preserve case, separators and opaque units while exposing only the payload.
    ///
    /// # Errors
    /// Return an unexpected refusal of the well-formed fixture.
    ///
    /// # Panics
    /// Panic if the capacity or the copied output differs.
    #[test]
    fn preserves_literal_path_and_ignores_unused_tail() -> Result<(), ProcessImageNameError> {
        let mut expected: Vec<u16> = r"C:\Fixture\NeVro".encode_utf16().collect();
        expected.extend_from_slice(&[0xd800, 0xffff]);
        expected.extend(".exe".encode_utf16());
        assert_eq!(expected.len(), 22);
        let observed: ReportedProcessImageName = read_process_image_name(
            |units: &mut [u16; PROCESS_IMAGE_NAME_BUFFER], count: &mut u32| {
                assert_eq!(*count, 1_024);
                units[..22].copy_from_slice(&expected);
                units[22] = 0;
                units[23] = u16::from(b'X');
                *count = 22;
                Ok(())
            },
        )?;
        assert_eq!(observed.as_wide(), expected);
        Ok(())
    }

    /// Admit both exact length boundaries and require the NUL at the declared end.
    ///
    /// # Errors
    /// Return an unexpected refusal of a healthy boundary.
    ///
    /// # Panics
    /// Panic if an invalid count or misplaced NUL is accepted.
    #[test]
    fn enforces_exclusive_length_and_exact_terminator() -> Result<(), ProcessImageNameError> {
        let maximum: ReportedProcessImageName = read_process_image_name(
            |units: &mut [u16; PROCESS_IMAGE_NAME_BUFFER], count: &mut u32| {
                units[..1_023].fill(u16::from(b'a'));
                units[1_023] = 0;
                *count = 1_023;
                Ok(())
            },
        )?;
        assert_eq!(maximum.as_wide(), &[u16::from(b'a'); 1_023]);

        let minimum: ReportedProcessImageName = read_process_image_name(
            |units: &mut [u16; PROCESS_IMAGE_NAME_BUFFER], count: &mut u32| {
                units[0] = u16::from(b'a');
                units[1] = 0;
                *count = 1;
                Ok(())
            },
        )?;
        assert_eq!(minimum.as_wide(), &[u16::from(b'a')]);

        let malformed: [(u32, &[usize]); 8] = [
            (0, &[0]),
            (1_024, &[1_023]),
            (1_025, &[1_023]),
            (u32::MAX, &[1_023]),
            (1, &[]),
            (1, &[2]),
            (2, &[1]),
            (2, &[1, 2]),
        ];
        for (reported, nul_indices) in malformed {
            let observed: Result<ReportedProcessImageName, ProcessImageNameError> =
                read_process_image_name(
                    |units: &mut [u16; PROCESS_IMAGE_NAME_BUFFER], count: &mut u32| {
                        units[0] = u16::from(b'a');
                        for &index in nul_indices {
                            units[index] = 0;
                        }
                        *count = reported;
                        Ok(())
                    },
                );
            assert!(matches!(observed, Err(ProcessImageNameError::Malformed)));
        }
        Ok(())
    }

    /// A native failure retains its code regardless of the returned count.
    ///
    /// # Panics
    /// Panic if a native failure is replaced by a decoder error.
    #[test]
    fn preserves_native_failure_before_buffer_validation() {
        for code in [0_u32, 5, 122, u32::MAX] {
            let observed: Result<ReportedProcessImageName, ProcessImageNameError> =
                read_process_image_name(
                    |units: &mut [u16; PROCESS_IMAGE_NAME_BUFFER], count: &mut u32| {
                        units[0] = 0;
                        *count = u32::MAX;
                        Err(code)
                    },
                );
            assert!(
                matches!(observed, Err(ProcessImageNameError::Native(actual)) if actual == code)
            );
        }
    }
}
