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
        if let Object { pos: Some(pos), player_mem: mem, .. } = obj {
            //Clear out the player memory if that spot has been seen again
            if mem.last_pos.is_some() {
                if visible.contains(mem.last_pos.as_ref().unwrap()) {
                    mem.seen = false;
                    mem.last_pos = None;
                }
            }

            'inner: for p in visible.iter() {
                if p == pos {
                    mem.seen = true;
                    mem.last_pos = Some(*p);
                    break 'inner;
                }
            }
        }
    }
}
