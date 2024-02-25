use std::{fs, thread, time::Duration};
use native_dialog::FileDialog;


pub fn open_folders ( loc: &str,  files_directory:&mut Vec<String>){

if let Ok(entries) = fs::read_dir(loc){
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
pub fn generate_audio (){


  let frequency = 440.0;
    // Sample rate (samples per second)
    let sample_rate = 44100;
    // Duration of the audio (seconds)
    let duration = 3;
    
    // Calculate the number of samples
    let num_samples = sample_rate * duration;

    // Generate and play the audio samples
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample_value = (t * frequency * 2.0 * std::f32::consts::PI).sin();
        
        // Print the sample value (simulating audio playback)
        println!("{}", sample_value);

        // Sleep to control the playback speed
        thread::sleep(Duration::from_micros((1.0 / sample_rate as f32 * 1_000_000.0) as u64));
    }
                      
                


}

pub fn show_directory (){
           match FileDialog::new().show_open_single_file() {
        Ok(Some(file_path)) => {
            // Get the directory part of the file path
            if let Some(parent_dir) = file_path.parent() {
                println!("Selected directory: {}", parent_dir.display());
            } else {
                println!("Could not determine directory from file path");
            }
        }
        Ok(None) => {
            println!("User canceled the file dialog");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    } 
      
}
