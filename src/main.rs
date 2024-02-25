use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::addr_of;

use crate::memory::ReadWriteProcessMemory;
use crate::process::{find_process, OpenCloseProcess};

#[cfg(target_os = "linux")]
mod linux_api;

#[cfg(target_os = "windows")]
mod windows_api;

mod memory;
mod os;
mod process;

fn main() {
    #[cfg(target_os = "linux")]
    let target_process_name = "test_ffi";

    #[cfg(target_os = "windows")]
    let target_process_name = "test_ffi.exe";

    let ammo = 16;
    let ammo_ptr = addr_of!(ammo) as *const c_void;
    let mut ammo_buffer: [i32; 1] = [0x0F_FF_FF_FF];

    // Find the process by its name.
    let mut process = find_process(String::from(target_process_name))
        .expect(std::format!("Process `{target_process_name}` not found").as_str());

    // Open the process with read and write access.
    process.open(true, true).expect("Failed to open process");

    // Write the new ammo quantity to the process memory.
    process
        .write(ammo_ptr, ammo_buffer.as_ptr().cast(), size_of::<i32>())
        .expect("Failed to write process memory");

    // Read the ammo quantity from the process memory.
    process
        .read(ammo_ptr, ammo_buffer.as_mut_ptr().cast(), size_of::<i32>())
        .expect("Failed to read process memory");

    // Close the process.
    process.close().expect("Failed to close process");

    println!("Ammo: {}", ammo_buffer[0]);
}
