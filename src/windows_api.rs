//! This module contains functions to manage processes.
#[cfg(target_os = "windows")]
pub mod process;
#[cfg(target_os = "windows")]
mod types;
