use bracket_lib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TileClass {
    Wall,
    Floor,
}
impl TileClass {
    pub fn does_collide(&self) -> bool {
        match self {
            Self::Wall => true,
            _ => false,
        }
    }
    pub fn does_blos(&self) -> bool {
        match self {
            Self::Wall => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileClass>,
    pub visible: Vec<bool>,
    pub revealed: Vec<bool>,
    pub objblocked: Vec<bool>,
}
impl Map {
    pub fn new(w: i32, h: i32) -> Map {
        Map {
            width: w,
            height: h,
            tiles: vec![TileClass::Floor; (w * h) as usize],
            visible: vec![false; (w * h) as usize],
            revealed: vec![false; (w * h) as usize],
            objblocked: vec![false; (w * h) as usize],
        }
    }

    //Grabs the vector index by encoding the X/Y values
    pub fn index(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }
    //The reverse of above
    pub fn point_from_idx(&self, idx: usize) -> Point {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        return Point::new(x, y);
    }

    //Same as index, but returns a None option if the index is out of bounds
    pub fn try_index(&self, x: i32, y: i32) -> Option<usize> {
        if !self.in_bounds(x, y) {
            return None;
        } else {
            return Some(self.index(x, y));
        }
    }
    //Checks if something is within the map's boundaries
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    //Checks to see if the tile at an index is walkable
    pub fn walkable(&self, x: i32, y: i32) -> bool {
        let idx = self.index(x, y);
        return self.in_bounds(x, y) && !self.tiles[idx].does_collide() && !self.objblocked[idx]
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].does_blos()
    }
}
