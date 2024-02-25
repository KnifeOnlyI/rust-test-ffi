/// This module contains types.
#[cfg(target_os = "windows")]
mod types;

/// This module contains functions to manage processes.
#[cfg(target_os = "windows")]
pub mod process;

/// This module contains functions to manage memory.
#[cfg(target_os = "windows")]
pub mod memory;
