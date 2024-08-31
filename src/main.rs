use eframe::egui;
mod functions;
mod structs;
use crate::structs::*;
mod gui;

fn main() {
    // Set default native options for the application
    let native_options = eframe::NativeOptions::default();
    // Run the native egui application with the specified title, options, and app creation function
    eframe::run_native("SteamReviewTemplate", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))))
        // Unwrap the result, which will panic if there's an error
        .unwrap();
}
