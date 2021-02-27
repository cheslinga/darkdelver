use crate::prelude::*;

pub fn process_fov(objects: &mut Vec<Object>, map: &mut Map) {
    let mut fovlist: Vec<usize> = Vec::new();
    let mut ailist: Vec<usize> = Vec::new();
    for (i,obj) in objects.iter().enumerate() {
        if obj.viewshed.is_some() {
            fovlist.push(i);
        }
    }
    for id in fovlist.iter() {
        if objects[*id].ai.is_some() {
            ailist.push(*id);
        }
    }

    for id in fovlist.iter() {
        let pos = objects[*id].pos.as_ref().unwrap().clone();
        let tag = objects[*id].tag.as_ref().unwrap().clone();
        let view = objects[*id].viewshed.as_mut().unwrap();

        if view.refresh {
            view.refresh = false;
            view.visible.clear();

            view.visible = field_of_view(pos, view.range, map);
            view.visible.retain(|p| {
                p.x >= 0 && p.x <= map.width - 1 && p.y >= 0 && p.y <= map.height - 1
            });

            if tag == ActorTag::Player {
                for t in map.visible.iter_mut() {
                    *t = false;
                }
                for v in view.visible.iter() {
                    let idx = map.index(v.x, v.y);
                    map.revealed[idx] = true;
                    map.visible[idx] = true;
                }
            }
        }
    }

    for id in ailist.iter() {
        //Process whether the AI target's position is within the viewshed
        //Using scopes here to keep the borrow checker happy and not have to write .as_ref().unwrap() for everything >-<
        let mut tgt_pos: Option<Point> = None;
        {
            let ai = objects[*id].ai.as_ref().unwrap();
            if let Some(tgt_id) = ai.target {
                tgt_pos = Some(objects[tgt_id].pos.as_ref().unwrap().clone());
            }
        }
        {
            let visible = objects[*id].viewshed.as_ref().unwrap().visible.to_vec();
            let ai = objects[*id].ai.as_mut().unwrap();

            if let Some(pos) = tgt_pos {
                if visible.contains(&pos) {
                    ai.tgt_heatmap.reset_to_single_node(&pos);
                } else {
                    ai.tgt_heatmap.clear_heat_area(&visible);
                }
            }
        }
    }
}