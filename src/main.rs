use serde_json;
use eframe::{egui, App, Storage};
mod menu;
mod program_table;


use program_table::program::Program;
//use menu::menu_bar::{MainMenu, Item};  // Import both MainMenu and Item

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct MowApp {
    global_state: GlobalState
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct GlobalState{
    counter: i32,
    programs: Vec<Program>
}


pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}



impl App for MowApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.label(format!("Counter: {}", &mut self.global_state.counter));
            if ui.button("Increment").clicked() {
                self.global_state.counter += 1;
            }
        });
    }

fn save(&mut self, storage: &mut dyn Storage) {
        if let Ok(state) = serde_json::to_string(self) {
            storage.set_string("app_state", state);
        }
    }

}

impl MowApp {
    fn load(storage: Option<&dyn Storage>) -> Self {
        if let Some(storage) = storage {
            if let Some(state) = storage.get_string("app_state") {
                if let Ok(app) = serde_json::from_str::<Self>(&state) {
                    return app;
                }
            }
        }
        Self::default()
    }
}



fn main() {
    let options = eframe::NativeOptions::default();
    if let Err(e) = 
        eframe::run_native(
             "MowApp",
            options,
            Box::new(|cc| {
                 Ok(Box::new(MowApp::load(cc.storage)))
             }),
        ){
            eprintln!("Failed to run the application: {}", e);
            std::process::exit(1);
        };
}
