#[cfg(target_os = "linux")]
use crate::linux_api::process::{enum_processes, get_process_name};
#[cfg(target_os = "linux")]
use crate::process::Process;

/// Get the list of all processes.
///
/// # Returns
/// A `Vec` of `Process` objects if the processes are found, otherwise an `Error`.
#[cfg(target_os = "linux")]
pub fn get_all() -> Result<Vec<Process>, String> {
    let r_process_ids = enum_processes();

    if r_process_ids.is_err() {
        return Err(r_process_ids.err().unwrap());
    }

    let mut processes = Vec::new();

    for process_id in r_process_ids.unwrap() {
        let r_process_name = get_process_name(process_id);

        if r_process_name.is_err() {
            println!(
                "Cannot get the process name because : `{}`",
                r_process_name.err().unwrap()
            );
            continue;
        }

        let process = Process {
            pid: process_id,
            name: r_process_name.unwrap(),
        };

        processes.push(process);
    }

    return Ok(processes);
}
