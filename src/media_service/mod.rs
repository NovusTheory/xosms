pub mod service_trait;

#[cfg(target_os = "windows")]
include!("windows.rs");
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
include!("linux/mod.rs");
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
include!("macos/mod.rs");
#[cfg(target_os = "macos")]
mod macos;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
include!("unsupported.rs");
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
mod unsupported;
