#![allow(unused)] // Errors are fine to be unused as they are typically constructed and passed to node making it unused in rust

#[cfg(target_os = "windows")]
#[derive(Debug)]
pub enum WindowsPlatformError {
  Core(windows_core::Error),
  HResult(windows_core::HRESULT),
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub enum LinuxPlatformError {
  ZBus(zbus::Error),
}

#[derive(Debug)]
pub enum PlatformBackendError {
  #[cfg(target_os = "windows")]
  Windows(WindowsPlatformError),
  #[cfg(target_os = "linux")]
  Linux(LinuxPlatformError),
}

#[derive(Debug)]
pub enum XosmsError {
  PlatformBackend(PlatformBackendError),
}

#[cfg(target_os = "windows")]
impl From<windows_core::Error> for XosmsError {
  fn from(value: windows_core::Error) -> Self {
    XosmsError::PlatformBackend(PlatformBackendError::Windows(WindowsPlatformError::Core(
      value,
    )))
  }
}

#[cfg(target_os = "linux")]
impl From<zbus::Error> for XosmsError {
  fn from(value: zbus::Error) -> Self {
    XosmsError::PlatformBackend(PlatformBackendError::Linux(LinuxPlatformError::ZBus(value)))
  }
}
