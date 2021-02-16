use crate::prelude::*;

pub fn update_player_memory(objects: &mut Vec<Object>) {
    let visible =
        if let Some(view) = &objects[0].viewshed {
            view.visible.to_vec()
        }
        else {
            Vec::new()
        };


    for obj in objects.iter_mut() {
        if let Some(pos) = &obj.pos {
            'inner: for p in visible.iter() {
                if p == pos {
                    obj.player_mem.seen = true;
                    obj.player_mem.last_pos = Some(*p);
                    break 'inner;
                }
            }
        }
    }
}