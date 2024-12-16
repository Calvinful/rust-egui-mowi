use crate::View;
use egui::{TextStyle, TextWrapMode};
use super::program::Program;
use egui;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct ProgramTable{
    program_list: Vec<Program>
}
impl View for ProgramTable { 
   fn ui(&mut self, ui: &mut egui::Ui){
        
   }
}


impl Default for ProgramTable{
    fn default() -> Self {
        Self{
            program_list: Vec::new()
        }
    }

}
