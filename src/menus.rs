use crate::prelude::*;

#[derive(Clone,Copy,PartialEq)]
pub enum MenuSelection { NewGame, LoadGame, Quit, Continue, SaveGame }

pub struct Menu {
    selections: Vec<MenuSelection>,
    pub current_selection: usize,
    pub processed_selection: Option<MenuSelection>
}
impl Menu {
    pub fn main_menu() -> Menu {
        Menu {
            selections: vec![MenuSelection::NewGame, MenuSelection::LoadGame, MenuSelection::Quit],
            current_selection: 0,
            processed_selection: None
        }
    }
    pub fn pause_menu() -> Menu {
        Menu {
            selections: vec![MenuSelection::Continue, MenuSelection::SaveGame, MenuSelection::LoadGame, MenuSelection::Quit],
            current_selection: 0,
            processed_selection: None
        }
    }
    pub fn cycle_selection_down(&mut self) {
        if self.current_selection != self.selections.len() - 1 {self.current_selection += 1}
        else {self.current_selection = 0}
    }
    pub fn cycle_selection_up(&mut self) {
        if self.current_selection != 0 {self.current_selection -= 1}
        else {self.current_selection = self.selections.len() - 1}
    }
    pub fn process_selection(&mut self) {
        self.processed_selection = Some(self.selections[self.current_selection])
    }
}
