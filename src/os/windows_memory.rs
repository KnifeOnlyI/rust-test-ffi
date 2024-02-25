use crate::process::Process;
use crate::windows_api::memory::{read_process_memory, write_process_memory};

/// Write to the process memory.
///
/// # Arguments
/// * process - The process to write to.
/// * address - The address to write to.
/// * buffer - The buffer to write from.
/// * size - The size of the buffer.
///
/// # Returns
/// The number of bytes written to the process memory.
pub fn write(
    process: &Process,
    address: *const std::ffi::c_void,
    buffer: *const std::ffi::c_void,
    size: usize,
) -> Result<usize, String> {
    let r_write_process_memory = write_process_memory(process.handle, address, buffer, size);

    return if r_write_process_memory.is_err() {
        Err(r_write_process_memory.unwrap_err().message())
    } else {
        Ok(r_write_process_memory.unwrap())
    };
}

/// Read from the process memory.
///
/// # Arguments
/// * process - The process to read from.
/// * address - The address to read from.
/// * buffer - The buffer to read into.
/// * size - The size of the buffer.
///
/// # Returns
/// The number of bytes read from the process memory.
pub fn read(
    process: &Process,
    address: *const std::ffi::c_void,
    buffer: *mut std::ffi::c_void,
    size: usize,
) -> Result<usize, String> {
    let r_write_process_memory = read_process_memory(process.handle, address, buffer, size);

    return if r_write_process_memory.is_err() {
        Err(r_write_process_memory.unwrap_err().message())
    } else {
        Ok(r_write_process_memory.unwrap())
    };
}
