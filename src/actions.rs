use crate::prelude::*;

impl Object {
    //Attempts to move an object, modifying its position
    pub fn try_move(&mut self, dest: Point, map: &Map) {
        if let Object { pos: Some(_), .. } = self {
            if !map.walkable(dest.x, dest.y) {
                return
            }
            else {
                self.pos = Some(dest);
                if let Object { viewshed: Some(view), .. } = self {
                    view.refresh = true
                }
            }
        }
        else {
            console::log("ERROR: Entity attempted to move without positional component.")
        }
    }
}

