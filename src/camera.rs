use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub const UI_CUTOFF: Point = Point { x: 24, y: 16 };

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}
impl Camera {
    pub fn new(pos: Point) -> Camera {
        Camera {
            min_x: pos.x - (CONSOLE_W - UI_CUTOFF.x) / 2,
            max_x: pos.x + (CONSOLE_W - UI_CUTOFF.x) / 2,
            min_y: pos.y - (CONSOLE_H - UI_CUTOFF.y) / 2,
            max_y: pos.y + (CONSOLE_H - UI_CUTOFF.y) / 2,
        }
    }
    pub fn move_camera(&mut self, pos: Point) {
        self.min_x = pos.x - (CONSOLE_W - UI_CUTOFF.x) / 2;
        self.max_x = pos.x + (CONSOLE_W - UI_CUTOFF.x) / 2;
        self.min_y = pos.y - (CONSOLE_H - UI_CUTOFF.y) / 2;
        self.max_y = pos.y + (CONSOLE_H - UI_CUTOFF.y) / 2;
    }
}
