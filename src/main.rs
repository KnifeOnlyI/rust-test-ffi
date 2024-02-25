use crate::process::find_process;

#[cfg(target_os = "linux")]
mod linux_api;
mod os;
mod process;
#[cfg(target_os = "windows")]
mod windows_api;

fn main() {
    #[cfg(target_os = "linux")]
        let target_process_name = "test_ffi";
    #[cfg(target_os = "windows")]
        let target_process_name = "test_ffi.exe";
    let r_process = find_process(String::from(target_process_name));

    if r_process.is_err() {
        println!("Process `{}` not found", target_process_name);
        return;
    }

    let process = r_process.unwrap();

    println!("[{}] {}", process.pid, process.name);
}
