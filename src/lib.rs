#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

// #[cfg(linux)]
// mod linux;
// #[cfg(macos)]
// mod macos;
#[cfg(windows)]
mod windows;
#[cfg(not(any(windows)))]
mod unsupported;