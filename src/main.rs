#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui; //Imports the rendering engine
use eframe::egui::Visuals; //Imports dark mode

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui", //app name
        options, //just leave this at options
        Box::new(|_cc| Box::new(MyApp::default())), //leave this as default
    );
}

struct MyApp {
    //Enter global values to be used with your app here
    insult: String,
}

impl Default for MyApp { //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            insult: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
            ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark
            egui::warn_if_debug_build(ui);

            ui.label("Hello and welcome to version 4 of the bully luna program!");
            ui.label("You can randomly generate an insult or write your own below to be sent to luna via discord!");

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Enter an insult: ");
                ui.text_edit_singleline(&mut self.insult);
            });
            
        });
    }
}
