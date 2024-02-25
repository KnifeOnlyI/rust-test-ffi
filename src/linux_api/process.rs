/// Enumerates all processes running on the system.
///
/// # Returns
/// If the function succeeds, the return value is a list of process identifiers.
pub fn enum_processes() -> Result<Vec<u32>, String> {
    let r_read_dir = std::fs::read_dir("/proc");

    if r_read_dir.is_err() {
        return Err(r_read_dir.err().unwrap().to_string());
    }

    let files = r_read_dir.unwrap();

    let mut process_ids = Vec::new();

    for file in files {
        let r_entry = file;

        if r_entry.is_err() {
            println!(
                "Cannot read a /proc file because : `{}`",
                r_entry.err().unwrap()
            );
            continue;
        }

        let entry = r_entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let file_name_str = path.file_name().unwrap().to_str().unwrap();

        if file_name_str.chars().all(char::is_numeric) {
            process_ids.push(file_name_str.parse::<u32>().unwrap());
        }
    }

    return Ok(process_ids);
}

/// Retrieves the name of the executable file for the specified process.
///
/// # Arguments
/// * pid - The process identifier.
///
/// # Returns
/// If the function succeeds, the return value is the name of the executable file for the specified process.
pub fn get_process_name(pid: u32) -> Result<String, String> {
    let r_file = std::fs::read_to_string(format!("/proc/{}/comm", pid));

    if r_file.is_err() {
        return Err(r_file.err().unwrap().to_string());
    }

    let file = r_file.unwrap();

    return Ok(file.trim().to_string());
}
