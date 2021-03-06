use crate::prelude::*;

pub fn update_blocked_tiles(objects: &Vec<Object>, map: &mut Map, floor: i32) {
    for b in map.objblocked.iter_mut() {
        *b = false;
    }

    for obj in objects.iter() {
        if let Object{ pos: Some(pos), .. } = obj {
            let block = &obj.block_tile;

            if *block && obj.floor == floor {
                let idx = map.index(pos.x, pos.y);
                map.objblocked[idx] = true;
            }
        }
    }
}
