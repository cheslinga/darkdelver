use bracket_lib::prelude::*;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd)]
#[repr(u16)]
pub enum TileClass {
    Wall = 1,
    Glass = 1024,
    Floor = 2048,
    DownStair = 4096,
}
impl TileClass {
    pub fn does_collide(&self) -> bool { *self < TileClass::Floor }
    pub fn does_blos(&self) -> bool { *self < TileClass::Glass }
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub fn from_copy(old_map: &Map) -> Map {
        Map {
            width: old_map.width,
            height: old_map.height,
            tiles: old_map.tiles.to_owned(),
            visible: old_map.visible.to_owned(),
            revealed: old_map.revealed.to_owned(),
            objblocked: old_map.objblocked.to_owned()
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
        return if !self.in_bounds(x, y) { None }
        else { Some(self.index(x, y)) }
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

    fn valid_exit(&self, pos: Point, delta: Point) -> Option<usize> {
        let dest = pos + delta;

        if self.in_bounds(dest.x, dest.y) {
            if self.walkable(dest.x, dest.y) {
                let idx = self.index(dest.x, dest.y);
                return Some(idx)
            }
        }
        return None
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point.x, point.y)
    }
}
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].does_blos()
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.point_from_idx(idx);

        if let Some(idx) = self.valid_exit(location, DL_LEFT)   { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, DL_RIGHT)  { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, DL_UP)     { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, DL_DOWN)   { exits.push((idx, 1.0)) }

        if let Some(idx) = self.valid_exit(location, DL_UP + DL_LEFT)       { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, DL_DOWN + DL_LEFT)     { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, DL_UP + DL_RIGHT)      { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, DL_DOWN + DL_RIGHT)    { exits.push((idx, 1.45)) }

        return exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        return DistanceAlg::Pythagoras.distance2d(
            self.point_from_idx(idx1),
            self.point_from_idx(idx2)
        )
    }
}
