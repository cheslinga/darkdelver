mod state;
mod input;
mod render;
mod object;
mod map;
mod camera;
mod spawn;

pub mod prelude {
    pub const CONSOLE_W: i32 = 80;
    pub const CONSOLE_H: i32 = 60;
    pub use bracket_lib::prelude::*;
    pub use crate::state::*;
    pub use crate::input::*;
    pub use crate::render::*;
    pub use crate::object::*;
    pub use crate::map::*;
    pub use crate::camera::*;
    pub use crate::spawn::*;
}
use crate::prelude::*;

fn main() {
    match main_loop(build_console(1280,720), State::init()) {
        Ok(_) => {},
        Err(e) => panic!("Could not initialize due to a fatal error:\n{}", e)
    }
}

fn build_console(w: i32, h: i32) -> BTerm {
    return BTermBuilder::new()
        .with_resource_path("res/")
        .with_font("font.png",16,16)
        .with_title("Citadel of Narthak")

        .with_tile_dimensions(16,16)
        .with_dimensions(w/16, h/16)

        .with_simple_console(CONSOLE_W,CONSOLE_H,"font.png")

        .with_vsync(true)
        .build()
        .unwrap();
}
