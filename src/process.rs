use std::fmt::Error;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HANDLE;
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

use crate::{os, windows_api};

/// Represents a process.
pub struct Process {
    #[cfg(target_os = "windows")]
    pub(crate) handle: HANDLE,

    /// The process ID.
    pub(crate) pid: u32,

    /// The process name.
    pub(crate) name: String,
}

pub trait OpenCloseProcess {
    fn open(&mut self, read: bool, write: bool) -> Result<(), String>;
    fn close(&self) -> Result<(), String>;
}

pub trait Drop {
    fn drop(&self);
}

impl OpenCloseProcess for Process {
    #[cfg(target_os = "windows")]
    fn open(&mut self, read: bool, write: bool) -> Result<(), String> {
        let access = if read && write {
            PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE
        } else if read {
            PROCESS_VM_READ
        } else if write {
            PROCESS_VM_OPERATION | PROCESS_VM_WRITE
        } else {
            return Err(String::from("No access specified"));
        };

        let r_handle = windows_api::process::open_process(access, self.pid);

        if r_handle.is_err() {
            return Err(r_handle.unwrap_err().message());
        }

        self.handle = r_handle.unwrap();

        return Ok(());
    }

    #[cfg(target_os = "windows")]
    fn close(&self) -> Result<(), String> {
        let r_close_handle = windows_api::process::close_handle(self.handle);

        if r_close_handle.is_err() {
            return Err(r_close_handle.unwrap_err().message());
        }

        return Ok(());
    }
}

impl Drop for Process {
    #[cfg(target_os = "windows")]
    fn drop(&self) {
        let _ = windows_api::process::close_handle(self.handle);
    }
}

/// Find a process by its name.
///
/// # Arguments
/// name - The name of the process to find.
///
/// # Returns
/// A `Process` object if the process is found, otherwise an `Error`.
pub fn find_process(name: String) -> Result<Process, Error> {
    let r_processes = os::process::get_all();

    if r_processes.is_err() {
        return Err(Error);
    }

    let processes = r_processes.unwrap();

    for process in processes {
        if process.name == name {
            return Ok(process);
        }
    }

    return Err(Error);
}
