use crate::prelude::*;

pub fn process_ai(objects: &mut Vec<Object>, map: &mut Map) {
    let player_pos = objects[0].pos.unwrap();
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
        basic_enemy_ai(id.0, objects, map, player_pos);
        update_blocked_tiles(objects, map);
    }
}

fn basic_enemy_ai(enemy_id: usize, objects: &mut Vec<Object>, map: &Map, player_pos: Point) {
    //Really basic shitty AI
    let enemy = &mut objects[enemy_id];
    let pos = enemy.pos.unwrap();

    if let Object { viewshed: Some(view), ..} = enemy {
        if view.visible.contains(&player_pos) {
            let mut dest: Point = pos;
            let targets = vec![map.index(player_pos.x, player_pos.y)];
            let dijkstra_map = DijkstraMap::new(CONSOLE_W, CONSOLE_H, &targets, map, 1024.0);

            if let Some(destidx) = DijkstraMap::find_lowest_exit(&dijkstra_map, map.index(pos.x, pos.y), map) {
                let distance = DistanceAlg::Pythagoras.distance2d(pos, player_pos);
                dest = if distance > 1.2 {
                    map.point_from_idx(destidx)
                } else {
                  player_pos
                };
            }
            if dest != pos { enemy.try_move(dest, map) }
        }
    }
}
