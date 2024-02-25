use audio::open_folders;
#[allow(dead_code)]
// use::std::env::args;

use eframe::egui;

use crate::audio::{generate_audio,show_directory};
// use std::fs;
mod audio;
#[derive(Default)]
struct MyDesktopApp {}
static LOC_DIR: &str = "/home/arwy/Documents/Projects/";
impl MyDesktopApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyDesktopApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
      let mut name = "Arwy";
      let mut age = 0;    
            // let  is_clicked: bool = false;
    let mut files_directory: Vec<String> = Vec::new();

ui.heading("MyDesktopApp");
ui.horizontal(|ui| {
    ui.label("Your name: ");
    ui.text_edit_singleline(&mut name);
});
if ui.button("Play Music").clicked(){
        generate_audio();
            };
ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
if ui.button("List Directory").clicked() {
open_folders(LOC_DIR,&mut files_directory);
                
              }
// ui.label(format!("Hello '{name}', age {age}"));
if files_directory.is_empty() == false {
                for files in files_directory {
                    ui.label(files);
                }
            }   
if ui.button("Open Directory").clicked() {
show_directory();          
            }

        });
   
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("MyApp", native_options, Box::new(|cc| Box::new(MyDesktopApp::new(cc))))
}

