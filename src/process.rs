use std::fmt::Error;

use crate::os;

/// Represents a process.
pub struct Process {
    /// The process ID.
    pub(crate) pid: u32,

    /// The process name.
    pub(crate) name: String,
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