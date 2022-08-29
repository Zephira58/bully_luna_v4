use eframe::egui; //Imports the rendering engine
use eframe::egui::Visuals; //Imports dark mode

mod api_handler; //Imports the API handler
use api_handler::*;

pub struct MyApp {
    //Enter global values to be used with your app here
    insult: String,
}

impl Default for MyApp {
    //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            insult: "".to_string(),
        }
    }
}

//A function that makes a get request from https://insult.mattbas.org/api/insult and returns it as a string

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
                ui.text_edit_multiline(&mut self.insult);
            });

            let generate_button = ui.button("Generate an insult");
            if generate_button.clicked() {
                self.insult = get_insult();
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Your insult:");
                ui.label(&self.insult);
            });

            let send_button = ui.button("Send insult to luna");
            if send_button.hovered() {
                ui.label("This is still a work in progress!");
            }
            if send_button.clicked() {
                send_message(self.insult.clone());
            }
        });
    }
}
