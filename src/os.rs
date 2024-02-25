//! This module contains functions to manage processes according to the target OS.
mod windows_process;

pub mod process {
    #[cfg(target_os = "windows")]
    pub use crate::os::windows_process::get_all;
}