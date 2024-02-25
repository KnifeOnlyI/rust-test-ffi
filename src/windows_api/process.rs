use std::mem::size_of;

use windows::core::Error;
use windows::Win32::Foundation::{BOOL, CloseHandle, HANDLE, HMODULE, MAX_PATH};
use windows::Win32::System::ProcessStatus;
use windows::Win32::System::ProcessStatus::{EnumProcesses, EnumProcessModules, EnumProcessModulesEx, LIST_MODULES_ALL};
use windows::Win32::System::Threading::{IsWow64Process, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

use crate::windows_api::types::DWORD_SIZE;

/// The default maximum number of processes that can be enumerated.
static DEFAULT_MAX_NB_PROCESSES: u32 = 1024;

/// Enumerates all processes running on the system.
///
/// # Returns
/// If the function succeeds, the return value is a list of process identifiers.
pub fn enum_processes(cb: Option<u32>) -> Result<Vec<u32>, Error> {
    let cb = cb.unwrap_or(DEFAULT_MAX_NB_PROCESSES);

    let mut lpidprocess = Vec::with_capacity(cb as usize);
    let mut lpcbneeded = 0;

    let r_processes = unsafe {
        EnumProcesses(
            lpidprocess.as_mut_ptr(),
            cb,
            &mut lpcbneeded)
    };

    if r_processes.is_err() {
        return Err(r_processes.unwrap_err());
    }

    unsafe { lpidprocess.set_len((lpcbneeded / DWORD_SIZE) as usize) };

    return Ok(lpidprocess);
}

/// Determines if the specified process is running under WOW64.
///
/// # Arguments
/// hprocess - A handle to the process.
///
/// # Returns
/// If the function succeeds, the return value is true if the process is running under WOW64.
pub fn is_wow64_process(hprocess: HANDLE) -> Result<BOOL, Error> {
    let mut wow64process = BOOL::from(false);

    let r_is_wow_64_process = unsafe { IsWow64Process(hprocess, &mut wow64process) };

    return if r_is_wow_64_process.is_err() {
        Err(r_is_wow_64_process.unwrap_err())
    } else {
        Ok(wow64process)
    };
}

/// Enumerates the modules associated with the specified process (for 32 bits).
///
/// # Arguments
/// hprocess - A handle to the process whose modules are to be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is an array of module handles.
pub fn enum_process_modules(hprocess: HANDLE) -> windows::core::Result<HMODULE> {
    let mut lphmodule = HMODULE::default();
    let mut lpcbneeded = 0;

    let r_enum_process_modules = unsafe {
        EnumProcessModules(
            hprocess,
            &mut lphmodule,
            size_of::<HMODULE>() as u32,
            &mut lpcbneeded,
        )
    };

    return if r_enum_process_modules.is_err() {
        Err(r_enum_process_modules.unwrap_err())
    } else {
        Ok(lphmodule)
    };
}

/// Enumerates the modules associated with the specified process (for 64 bits).
///
/// # Arguments
/// hprocess - A handle to the process whose modules are to be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is an array of module handles.
pub fn enum_process_modules_ex(hprocess: HANDLE) -> windows::core::Result<HMODULE> {
    let mut lphmodule = HMODULE::default();
    let mut lpcbneeded = 0;

    let r_enum_process_modules = unsafe {
        EnumProcessModulesEx(
            hprocess,
            &mut lphmodule,
            size_of::<HMODULE>() as u32,
            &mut lpcbneeded,
            LIST_MODULES_ALL,
        )
    };

    return if r_enum_process_modules.is_err() {
        Err(r_enum_process_modules.unwrap_err())
    } else {
        Ok(lphmodule)
    };
}

/// Retrieves the base name of the specified module (wide character).
///
/// # Arguments
/// hprocess - A handle to the process that contains the module.
/// hmodule - A handle to the module.
///
/// # Returns
/// If the function succeeds, the return value is the base name of the module.
pub fn get_module_base_name_w(hprocess: HANDLE, hmodule: HMODULE) -> windows::core::Result<String> {
    let mut lpbasename = [0; MAX_PATH as usize];

    let module_base_name_length = unsafe {
        ProcessStatus::GetModuleBaseNameW(
            hprocess,
            hmodule,
            &mut lpbasename,
        )
    };

    return if module_base_name_length == 0 {
        Err(Error::from_win32())
    } else {
        Ok(String::from_utf16_lossy(&lpbasename[0..module_base_name_length as usize]))
    };
}

/// Opens the specified process and returns a handle to it.
///
/// # Arguments
/// dwprocessid - The identifier of the local process to be opened.
///
/// # Returns
/// If the function succeeds, the return value is an open handle to the specified process.
pub fn open_process(dwprocessid: u32) -> windows::core::Result<HANDLE> {
    return unsafe {
        OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            dwprocessid,
        )
    };
}

/// Closes the specified handle.
///
/// # Arguments
/// hobject - A handle to an open object.
///
/// # Returns
/// If the function succeeds, the return value is nonzero.
pub fn close_handle(hobject: HANDLE) -> windows::core::Result<()> {
    return unsafe { CloseHandle(hobject) };
}