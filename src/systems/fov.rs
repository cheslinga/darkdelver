use crate::prelude::*;

pub fn process_fov(objects: &mut Vec<Object>, map: &mut Map) {
    for obj in objects.iter_mut() {
        if let Object{ viewshed: Some(_), .. } = obj {
            let view = &mut obj.viewshed.as_mut().unwrap();
            let pos = &obj.pos.unwrap();

            if view.refresh {
                view.refresh = false;
                view.visible.clear();

                view.visible = field_of_view(*pos, view.range, map);
                view.visible
                    .retain(|p| p.x >= 0 && p.x <= map.width - 1 && p.y >= 0 && p.y <= map.height - 1);

                if let Object{tag: Some(ActorTag::Player), ..} = obj {
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
    }
}
