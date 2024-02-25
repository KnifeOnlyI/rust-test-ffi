use std::ffi::c_void;

use windows::core::Error;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

/// Read and copy the content at the specified address in the memory of the specified process in the specified buffer.
///
/// # Arguments
/// * hprocess - A handle to the process with memory that is being read.
/// * lpbaseaddress - A pointer to the base address in the specified process to read from.
/// * lpbuffer - A pointer to a buffer that receives the contents from the address space of the specified process.
/// * nsize - The number of bytes to be read from the specified process.
///
/// # Returns
/// If the function succeeds, the return value is the number of bytes read from the specified process.
pub fn read_process_memory(
    hprocess: HANDLE,
    lpbaseaddress: *const c_void,
    lpbuffer: *mut c_void,
    nsize: usize,
) -> Result<usize, Error> {
    let mut lpnumberofbytesread = 0;

    let r_read_process_memory = unsafe {
        ReadProcessMemory(
            hprocess,
            lpbaseaddress,
            lpbuffer,
            nsize,
            Some(&mut lpnumberofbytesread),
        )
    };

    return if r_read_process_memory.is_err() {
        Err(r_read_process_memory.unwrap_err())
    } else {
        Ok(lpnumberofbytesread)
    };
}

/// Write the specified buffer in the memory of the specified process at the specified address.
///
/// # Arguments
/// * hprocess - A handle to the process with memory that is being written to.
/// * lpbaseaddress - A pointer to the base address in the specified process to write to.
/// * lpbuffer - A pointer to a buffer that contains the data to be written in the address space of the specified process.
/// * nsize - The number of bytes to be written to the specified process.
///
/// # Returns
/// If the function succeeds, the return value is the number of bytes written to the specified process.
pub fn write_process_memory(
    hprocess: HANDLE,
    lpbaseaddress: *const c_void,
    lpbuffer: *const c_void,
    nsize: usize,
) -> Result<usize, Error> {
    let mut lpnumberofbyteswritten = 0;

    let r_write_process_memory = unsafe {
        WriteProcessMemory(
            hprocess,
            lpbaseaddress,
            lpbuffer,
            nsize,
            Some(&mut lpnumberofbyteswritten),
        )
    };

    return if r_write_process_memory.is_err() {
        Err(r_write_process_memory.unwrap_err())
    } else {
        Ok(lpnumberofbyteswritten)
    };
}
