use std::time::Duration;

use eframe::egui::{self, Visuals, Window};
use egui_notify::{Anchor, Toast, Toasts};

mod api_handler; //Imports the API handler
use api_handler::*;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    insult: String,
    closable: bool,
    duration: f32,
    mention: bool,

    #[serde(skip)]
    insult_promise: Option<poll_promise::Promise<String>>,
    #[serde(skip)]
    insult_send_promise: Option<poll_promise::Promise<()>>,

    #[serde(skip)]
    toasts: Toasts,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            insult: "".to_string(),
            toasts: Toasts::default().with_anchor(Anchor::TopRight),
            closable: true,
            duration: 3.5,
            mention: true,

            insult_promise: None,
            insult_send_promise: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Window::new("bully_luna_v4").show(ctx, |ui| {
            ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
            ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark
            egui::warn_if_debug_build(ui);

            ui.horizontal(|ui| {
                ui.label("Current build:");
                ui.hyperlink_to("1.2.2", "https://github.com/Xanthus58/bully_luna_v4");
            });

            ui.add_space(8.0);

            let cb = |t: &mut Toast| {
                //Callback for the toast
                t.set_closable(self.closable)
                    .set_duration(Some(Duration::from_millis((1000. * self.duration) as u64)));
            };

            self.insult = self.insult.replace("\n", "");

            ui.label("Hello and welcome to version 4 of the bully luna program!");
            ui.label(
                "You can randomly generate an insult or write your own below to be sent to luna!",
            );

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Enter an insult: ");
                ui.text_edit_multiline(&mut self.insult);
            });

            ui.horizontal(|ui| {
                ui.label("Your message:");
                ui.label(&self.insult);
            });

            ui.separator();

            ui.horizontal(|ui| {
                let generate_button = ui.button("Generate an insult");
                if generate_button.clicked() {
                    self.insult_promise = Some(get_insult());
                }

                if let Some(promise) = &mut self.insult_promise {
                    match promise.ready() {
                        Some(insult) => {
                            cb(self.toasts.success("Generation Successful!")); //Sends a success toast
                            self.insult = insult.clone();
                            self.insult_promise = None;
                        }
                        None => {
                            ui.spinner();
                        }
                    }
                }

                let send_button = ui.button("Send message to luna");
                if send_button.clicked() {
                    self.insult_send_promise = Some(send_message(&self.insult, self.mention));
                }

                if let Some(promise) = &mut self.insult_send_promise {
                    match promise.ready() {
                        Some(()) => {
                            cb(self.toasts.success("Message Sent!"));
                            self.insult = "".to_string();
                            self.insult_send_promise = None;
                        }
                        None => {
                            ui.spinner();
                        }
                    }
                }

                ui.checkbox(&mut self.mention, "Mention luna?")
            });
            self.toasts.show(ctx); // Requests to render toasts
        });
    }
}
