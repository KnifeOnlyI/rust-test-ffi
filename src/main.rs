use std::env;

use crate::process::find_process;

mod windows_api;
mod process;
mod os;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_process_name = &args[1];
    let process = find_process(String::from(target_process_name)).expect("Process not found");

    println!("[{}] {}", process.pid, process.name);
}