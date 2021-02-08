use crate::prelude::*;

pub fn update_player_memory(objects: &mut Vec<Object>) {
    let player_visible = objects[0].viewshed.as_ref().unwrap().visible.to_owned();

    for obj in objects.iter_mut() {
        let pos = obj.pos.as_ref().unwrap();

        'inner: for p in player_visible.iter() {
            if p == pos {
                obj.player_mem.seen = true;
                obj.player_mem.last_pos = Some(*p);
                break 'inner;
            }
        }
    }
}