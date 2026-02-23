
fn main() {
    let icon = if cfg!(feature = "1") {
        "assets/Antid0te.ico"
    } else if cfg!(feature = "2") {
        "assets/note.ico"
    } else {
        "assets/icon.ico"
    };

    if let Err(e) = winres::WindowsResource::new().set_icon(icon).compile() {
        println!("cargo:warning={}", e);
    }
}

