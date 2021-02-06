#![windows_subsystem = "windows"]
mod camera;
mod input;
mod map;
mod menus;
mod object;
mod render;
mod saves;
mod spawn;
mod state;
mod systems;
mod actions;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::input::*;
    pub use crate::map::*;
    pub use crate::menus::*;
    pub use crate::object::*;
    pub use crate::render::*;
    pub use crate::saves::*;
    pub use crate::spawn::*;
    pub use crate::state::*;
    pub use crate::systems::*;
    pub use crate::actions::*;
    pub use bracket_lib::prelude::*;
    pub use std::cmp::Reverse;

    //Consts to define the drawable console space
    pub const CONSOLE_W: i32 = 80;
    pub const CONSOLE_H: i32 = 60;

    //Constant point values used as directional indicators
    pub const DL_LEFT: Point = Point { x: -1, y: 0 };
    pub const DL_RIGHT: Point = Point { x: 1, y: 0 };
    pub const DL_UP: Point = Point { x: 0, y: -1 };
    pub const DL_DOWN: Point = Point { x: 0, y: 1 };

    //Type alias for a vec of array indices and initiative values
    pub type InitList = Vec<(usize, u8)>;

    pub trait InitListTrait {
        fn add_object(&mut self, id: usize, init: u8);
        fn sort(&mut self);
    }
    impl InitListTrait for InitList {
        //Adds a new object to the list
        fn add_object(&mut self, id: usize, init: u8) {
            self.push((id, init));
        }
        //Sorts by descending initiative order
        fn sort(&mut self) {
            self.sort_by_key(|a| Reverse(a.1));
        }
    }
}
use crate::prelude::*;

fn main() {
    match main_loop(build_console(1024, 768), State::init()) {
        Ok(_) => {}
        Err(e) => panic!("Could not initialize due to a fatal error:\n{}", e),
    }
}

fn build_console(w: i32, h: i32) -> BTerm {
    return BTermBuilder::new()
        .with_resource_path("res/")
        .with_font("font.png", 16, 16)
        .with_title("Darkdelver")
        .with_tile_dimensions(16, 16)
        .with_dimensions(w / 16, h / 16)
        .with_simple_console(CONSOLE_W, CONSOLE_H, "font.png")
        .with_sparse_console_no_bg(CONSOLE_W, CONSOLE_H, "font.png")
        .with_vsync(true)
        .build()
        .unwrap();
}
