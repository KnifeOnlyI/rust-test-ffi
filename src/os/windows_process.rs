use windows::Win32::System::Threading::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

#[cfg(target_os = "windows")]
use crate::process::Process;
#[cfg(target_os = "windows")]
use crate::windows_api::process;

/// Get the list of all processes.
///
/// # Returns
/// A `Vec` of `Process` objects if the processes are found, otherwise an `Error`.
#[cfg(target_os = "windows")]
pub fn get_all() -> Result<Vec<Process>, String> {
    let r_process_ids = process::enum_processes(None);

    if r_process_ids.is_err() {
        return Err(r_process_ids.unwrap_err().message());
    }

    let mut processes = Vec::new();

    for process_id in r_process_ids.unwrap() {
        let r_handle =
            process::open_process(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, process_id);

        if r_handle.is_err() {
            continue;
        }

        let handle = r_handle.unwrap();

        let r_is_wow_64_process = process::is_wow64_process(handle);

        if r_is_wow_64_process.is_err() {
            println!(
                "Cannot determine if process {} is 32 or 64 bits",
                process_id
            );
            continue;
        }

        let is_wow_64_process = r_is_wow_64_process.unwrap().as_bool();
        let r_process_modules = if is_wow_64_process {
            process::enum_process_modules_ex(handle)
        } else {
            process::enum_process_modules(handle)
        };

        if r_process_modules.is_err() {
            println!("Cannot enumerate modules for process {}", process_id);
            continue;
        }

        let hmodule = r_process_modules.unwrap();
        let r_module_base_name = process::get_module_base_name_w(handle, hmodule);

        if r_module_base_name.is_err() {
            println!("Cannot get module base name for process {}", process_id);
            continue;
        }

        let process_name = r_module_base_name.unwrap();

        processes.push(Process {
            handle,
            pid: process_id,
            name: process_name,
        });

        if process::close_handle(handle).is_err() {
            println!("Cannot close handle for process {}", process_id);
        }
    }

    return Ok(processes);
}
