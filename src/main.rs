use std::{fs};


use eframe::egui;
// use std::fs;
#[allow(dead_code)]
#[derive(Default)]
struct MyDesktopApp {}

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
    let mut files_directory = Vec::new();

ui.heading("MyDesktopApp");
ui.horizontal(|ui| {
    ui.label("Your name: ");
    ui.text_edit_singleline(&mut name);
});
ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
if ui.button("List Directory").clicked() {
                
                if let Ok(entries) = fs::read_dir("/home/arwy/Documents/"){
                    for entry in entries {
                        let Ok(entry) = entry else { continue };
                        
                        let paths = entry.path();
                        let display_paths = paths.clone();
                        files_directory.push(display_paths.display().to_string());
                        // println!("{}",paths.display());
                    }
                }else{
                    println!("Failed open directory")
                }
}
ui.label(format!("Hello '{name}', age {age}"));
if files_directory.is_empty() == false {
                for files in files_directory {
                    ui.label(files);
                }
            }   
if ui.button("Open Directory").clicked() {
                
                if let Ok(entries) = fs::read_dir("/home/arwy/Documents/"){
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let paths = entry.path();
                            println!("{}",paths.display());
                        }
                    }
                }else{
                    println!("Failed open directory")
                }
}

        });
   
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("MyApp", native_options, Box::new(|cc| Box::new(MyDesktopApp::new(cc))))
}

