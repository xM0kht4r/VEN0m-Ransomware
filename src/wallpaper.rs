use anyhow::{Result, Context, bail};
use std::io::Write;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{
    SystemParametersInfoW,
    SPI_SETDESKWALLPAPER,
    SPIF_UPDATEINIFILE,
    SPIF_SENDCHANGE,
};

const WALL: &[u8] = include_bytes!(r"../assets/wallpaper.jpg");

pub fn set_wallpaper() -> Result<()>{
    let image_path = load(WALL)?;

    let wide: Vec<u16> = OsStr::new(&image_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            wide.as_ptr() as _,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if result == 0 {
            bail!("Failed to set wallpaper");
        }
    }
    Ok(())
}

// Drop the wallpaper to disk
fn load(image_bytes: &[u8]) -> Result<PathBuf> {

        let img_path = std::env::temp_dir().join("MicrosoftUpdate11.03.jpg");
        if !img_path.exists()
        // The written image has to be out of scope in order to use it
        {
            let mut file = std::fs::File::create(&img_path).context("[!] Failed to create image file!")?;
            file.write_all(image_bytes).context("[!] Failed to data to file")?;
        }

        Ok(img_path)
}

