#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod egui;
use egui::*;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bully Luna V.4", //app name
        options, //just leave this at options
        Box::new(|_cc| Box::new(MyApp::default())), //leave this as default
    );
}