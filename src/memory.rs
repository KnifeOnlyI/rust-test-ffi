use std::ffi::c_void;

use crate::os;
use crate::process::Process;

pub trait ReadWriteProcessMemory {
    /// Reads the memory of the process.
    ///
    /// # Arguments
    /// address - The address to read from.
    /// buffer - The buffer to read into.
    /// size - The size of the buffer.
    ///
    /// # Returns
    /// The number of bytes read.
    fn read(
        &self,
        address: *const c_void,
        buffer: *mut c_void,
        size: usize,
    ) -> Result<usize, String>;

    /// Writes the memory of the process.
    ///
    /// # Arguments
    /// address - The address to write to.
    /// buffer - The buffer to write from.
    /// size - The size of the buffer.
    ///
    /// # Returns
    /// The number of bytes written.
    fn write(
        &self,
        address: *const c_void,
        buffer: *const c_void,
        size: usize,
    ) -> Result<usize, String>;
}

impl ReadWriteProcessMemory for Process {
    fn read(
        &self,
        address: *const c_void,
        buffer: *mut c_void,
        size: usize,
    ) -> Result<usize, String> {
        let r_read = os::process::read(self, address, buffer, size);

        if r_read.is_err() {
            return Err(r_read.unwrap_err());
        }

        return Ok(r_read.unwrap());
    }

    fn write(
        &self,
        address: *const c_void,
        buffer: *const c_void,
        size: usize,
    ) -> Result<usize, String> {
        let r_write = os::process::write(self, address, buffer, size);

        if r_write.is_err() {
            return Err(r_write.unwrap_err());
        }

        return Ok(r_write.unwrap());
    }
}
