use crate::prelude::*;

pub fn update_blocked_tiles(objects: &Vec<Object>, map: &mut Map) {
    for b in map.objblocked.iter_mut() {
        *b = false;
    }

    for obj in objects.iter() {
        if let Object{ pos: Some(_), .. } = obj {
            let pos = &obj.pos.unwrap();
            let block = &obj.block_tile;

            if *block {
                let idx = map.index(pos.x, pos.y);
                map.objblocked[idx] = true;
            }
        }
    }
}
