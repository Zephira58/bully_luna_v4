#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod egui;
use egui::*;

pub const APP_NAME: &str = "Bully Luna v.4";

fn main() {
    println!("quick test for github workflows on PR");
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,                                   //app name
        options,                                    //just leave this at options
        Box::new(|_cc| Box::new(MyApp::default())), //leave this as default
    );
}
