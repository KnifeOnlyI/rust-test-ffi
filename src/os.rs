//! This module contains functions to manage processes according to the target OS.
mod linux_process;
mod windows_memory;
mod windows_process;

pub mod process {
    #[cfg(target_os = "windows")]
    pub use crate::os::windows_process::get_all;

    #[cfg(target_os = "windows")]
    pub use crate::os::windows_memory::write;

    #[cfg(target_os = "windows")]
    pub use crate::os::windows_memory::read;

    #[cfg(target_os = "linux")]
    pub use crate::os::linux_process::get_all;
}
