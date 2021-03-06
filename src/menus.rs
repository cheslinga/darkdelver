use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MenuSelection {
    NewGame,
    LoadGame,
    Quit,
    Continue,
    SaveGame
}

pub struct Menu {
    selections: Vec<MenuSelection>,
    pub current_selection: usize,
    pub processed_selection: Option<MenuSelection>,
}
impl Menu {
    pub fn main_menu() -> Menu {
        Menu {
            selections: vec![
                MenuSelection::NewGame,
                MenuSelection::LoadGame,
                MenuSelection::Quit,
            ],
            current_selection: 0,
            processed_selection: None,
        }
    }
    pub fn pause_menu() -> Menu {
        Menu {
            selections: vec![
                MenuSelection::Continue,
                MenuSelection::SaveGame,
                MenuSelection::LoadGame,
                MenuSelection::Quit,
            ],
            current_selection: 0,
            processed_selection: None,
        }
    }
    pub fn cycle_selection_down(&mut self) {
        if self.current_selection != self.selections.len() - 1 {
            self.current_selection += 1
        } else {
            self.current_selection = 0
        }
    }
    pub fn cycle_selection_up(&mut self) {
        if self.current_selection != 0 {
            self.current_selection -= 1
        } else {
            self.current_selection = self.selections.len() - 1
        }
    }
    pub fn process_selection(&mut self) {
        self.processed_selection = Some(self.selections[self.current_selection])
    }
}

pub fn batch_main_menu(menu: &Menu) {
    let mut bgbatch = DrawBatch::new();
    let mut textbatch = DrawBatch::new();
    bgbatch.target(OBJ_LAYER);
    textbatch.target(TXT_LAYER);

    textbatch.print_color_centered(CONSOLE_H / 4, "Darkdelver", ColorPair::new(RED, BLACK));

    textbatch.print_color_centered(
        CONSOLE_H - 3,
        "Copyright (C) 2021, Cole Heslinga",
        ColorPair::new(WHITE, BLACK),
    );

    let unselected: ColorPair = ColorPair::new(WHITE, BLACK);
    let selected: ColorPair = ColorPair::new(YELLOW, GREY10);

    let mut newgame_pair: ColorPair = unselected;
    let mut loadgame_pair: ColorPair = unselected;
    let mut quit_pair: ColorPair = unselected;

    if menu.current_selection == 0 {
        newgame_pair = selected;
    } else if menu.current_selection == 1 {
        loadgame_pair = selected;
    } else if menu.current_selection == 2 {
        quit_pair = selected;
    }

    textbatch.print_color(
        Point::new(CONSOLE_W - 10, CONSOLE_H / 4 + 3),
        "New Game",
        newgame_pair,
    );
    textbatch.print_color(
        Point::new(CONSOLE_W - 10, CONSOLE_H / 4 + 5),
        "Load Game",
        loadgame_pair,
    );
    textbatch.print_color(
        Point::new(CONSOLE_W - 10, CONSOLE_H / 4 + 7),
        "Quit",
        quit_pair,
    );

    bgbatch.fill_region(Rect::with_exact(0,0,CONSOLE_W * 2,CONSOLE_H), ColorPair::new(BLACK,BLACK), 0);
    bgbatch.submit(0).expect("Failed to batch menu draw");
    textbatch.submit(0).expect("Failed to batch menu draw");
}
pub fn batch_pause_menu(menu: &Menu) {
    let mut textbatch = DrawBatch::new();
    textbatch.target(TXT_LAYER);

    let unselected: ColorPair = ColorPair::new(WHITE, BLACK);
    let selected: ColorPair = ColorPair::new(YELLOW, GREY10);

    let mut continue_pair: ColorPair = unselected;
    let mut savegame_pair: ColorPair = unselected;
    let mut loadgame_pair: ColorPair = unselected;
    let mut quit_pair: ColorPair = unselected;

    if menu.current_selection == 0 {
        continue_pair = selected;
    } else if menu.current_selection == 1 {
        savegame_pair = selected;
    } else if menu.current_selection == 2 {
        loadgame_pair = selected;
    } else if menu.current_selection == 3 {
        quit_pair = selected;
    }

    textbatch.print_color(
        Point::new(CONSOLE_W / 2 - 5, CONSOLE_H / 4 + 1),
        "Continue",
        continue_pair,
    );
    textbatch.print_color(
        Point::new(CONSOLE_W / 2 - 5, CONSOLE_H / 4 + 3),
        "Save Game",
        savegame_pair,
    );
    textbatch.print_color(
        Point::new(CONSOLE_W / 2 - 5, CONSOLE_H / 4 + 5),
        "Load Game",
        loadgame_pair,
    );
    textbatch.print_color(
        Point::new(CONSOLE_W / 2 - 5, CONSOLE_H / 4 + 7),
        "Quit",
        quit_pair,
    );

    textbatch.submit(0).expect("Failed to batch menu draw");
}