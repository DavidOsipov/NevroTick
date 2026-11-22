// SPDX-License-Identifier: GPL-2.0-only
//! Native observation adapters linked into their owning Windows process.
//!
//! The portable platform crate retains its separate service lifecycle contract.

#![deny(unsafe_code)]
#![cfg_attr(
    not(windows),
    doc = r"
The subscription and local clock readers are available on Windows builds.

```compile_fail,E0432
use windows_platform_native::ResumeWatch;
```

```compile_fail,E0432
use windows_platform_native::read_clock_coordinates;
```

```compile_fail,E0432
use windows_platform_native::read_leap_presentation;
```

```compile_fail,E0432
use windows_platform_native::PeerProcess;
```
"
)]

#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod clock_coordinates;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod dns_address;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod leap_presentation;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod package_full_name;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod process_image_name;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod resume_slot;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod service_configuration;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod service_peer;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod service_security;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod service_status;
#[cfg(windows)]
mod windows_adjustment;
#[cfg(windows)]
mod windows_clock;
#[cfg(windows)]
mod windows_dns;
#[cfg(windows)]
mod windows_leap_presentation;
#[cfg(windows)]
mod windows_peer_process;
#[cfg(windows)]
mod windows_resume;
#[cfg(windows)]
mod windows_service_configuration;
#[cfg(windows)]
mod windows_service_security;
#[cfg(windows)]
mod windows_service_status;

#[cfg(any(windows, test))]
pub use clock_coordinates::{ClockCoordinates, ClockReadError};
#[cfg(any(windows, test))]
pub use dns_address::{
    DnsAddressError, DnsAddressFamily, DnsAddresses, DnsCancellationRequest, DnsNameError,
    DnsQueryName,
};
#[cfg(any(windows, test))]
pub use leap_presentation::{LeapPresentation, LeapPresentationError};
#[cfg(any(windows, test))]
pub use package_full_name::{PackageFullNameError, ReportedPackageFullName};
#[cfg(any(windows, test))]
pub use process_image_name::{ProcessImageNameError, ReportedProcessImageName};
#[cfg(any(windows, test))]
pub use service_configuration::ConfigurationStringError;
#[cfg(windows)]
pub use service_peer::{ObservedServiceProcess, ServiceProcessObservationError};
#[cfg(any(windows, test))]
pub use service_security::{ReportedDacl, ReportedServiceSecurity, ServiceSecurityError};
#[cfg(any(windows, test))]
pub use service_status::{
    InvalidServiceKey, ReportedServiceStatus, RunningServiceRefusal, ServiceKey,
};
#[cfg(windows)]
pub use windows_adjustment::{ClockAdjustment, read_clock_adjustment};
#[cfg(windows)]
pub use windows_clock::read_clock_coordinates;
#[cfg(windows)]
pub use windows_dns::{DnsAddressQuery, dns_worker_thread_exited};
#[cfg(windows)]
pub use windows_leap_presentation::read_leap_presentation;
#[cfg(windows)]
pub use windows_peer_process::{PeerProcess, PeerProcessError};
#[cfg(windows)]
pub use windows_resume::{ResumeRegistrationError, ResumeUnregistrationError, ResumeWatch};
#[cfg(windows)]
pub use windows_service_configuration::{ReportedServiceConfiguration, ServiceConfigurationQuery};
#[cfg(windows)]
pub use windows_service_security::ServiceSecurityQuery;
#[cfg(windows)]
pub use windows_service_status::{ServiceQueryError, ServiceStatusQuery};
