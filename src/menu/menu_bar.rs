use crate::View;
use egui;
#[derive( serde::Serialize, serde::Deserialize)]
pub struct Item {
    name: String,
    page_link: String
}

#[derive( serde::Serialize, serde::Deserialize)]
pub struct MainMenu {
    items: Vec<Item>
}

impl View for MainMenu {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            MainMenu::nested_menus(ui);
        });

        ui.menu_button("Edit", |ui| {
            if ui.button("Cut").clicked() {
                ui.close_menu();
            }
            if ui.button("Copy").clicked() {
                ui.close_menu();
            }
            if ui.button("Paste").clicked() {
                ui.close_menu();
            }
        });

        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                ui.close_menu();
            }
        });
    }
}

impl MainMenu {
    pub(super) fn nested_menus(ui: &mut egui::Ui) {
        ui.set_max_width(200.0);
        if ui.button("Open…").clicked() {
            ui.close_menu();
        }

        ui.menu_button("Recent Files", |ui| {
            ui.menu_button("Project 1", |ui| {
                if ui.button("Open").clicked() {
                    ui.close_menu();
                }
                ui.button("Remove from list");
            });

            ui.menu_button("Project 2", |ui| {
                if ui.button("Open").clicked() {
                    ui.close_menu();
                }
                ui.button("Remove from list");
            });

            ui.button("Clear Recent");
            if ui.button("Browse...").clicked() {
                ui.close_menu();
            }
        });

        if ui.button("Save").clicked() {
            ui.close_menu();
        }
        if ui.button("Save As...").clicked() {
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Exit").clicked() {
            ui.close_menu();
        }
    }
}





