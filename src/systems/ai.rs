use crate::prelude::*;

pub fn process_ai(objects: &mut Vec<Object>, map: &Map) {
    let mut proclist: Vec<(usize, u8)> = Vec::new();

    for (id, obj) in objects.iter().enumerate() {
        if let Object{ tag: Some(tag), initiative: Some(init), .. } = obj {
            if *tag == ActorTag::Enemy {
                proclist.push((id, *init));
            }
        }
    }

    proclist.sort_by(|a,b| a.0.cmp(&b.0));
    for id in proclist.iter() {
        basic_enemy_ai(id.0, objects, map);
    }
}

fn basic_enemy_ai(enemy_id: usize, objects: &mut Vec<Object>, map: &Map) {
    //Really basic shitty AI
    let enemy = &mut objects[enemy_id];
    let pos = enemy.pos.unwrap();
    let dest = pos + Point::new(1,0);

    enemy.try_move(dest, map);
}
