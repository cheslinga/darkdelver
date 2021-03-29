use crate::prelude::*;

pub fn process_ai(objects: &mut Vec<Object>, map: &mut Map, floor: i32, rng: &mut RandomNumberGenerator) {
    let (player, all) = objects.split_at_mut(1);
    let player_pos = player[0].pos.unwrap();
    let mut proclist: InitList = InitList::new();

    for (id, obj) in all.iter().enumerate() {
        if let Object{ tag: Some(tag), initiative: Some(init), .. } = obj {
            if *tag == ActorTag::Enemy {
                proclist.add_object(id, *init);
            }
        }
    }
    proclist.sort();

    for unit in proclist.iter() {
        basic_enemy_ai(unit.0, objects, map, rng, player_pos);
        update_blocked_tiles(objects, map, floor);
    }
}

fn basic_enemy_ai(enemy_id: usize, objects: &mut Vec<Object>, map: &Map, rng: &mut RandomNumberGenerator, player_pos: Point) {
    let (player, all) = &mut objects.split_at_mut(1);
    let enemy = &mut all[enemy_id];
    let player = &mut player[0];
    let pos = enemy.pos.unwrap();

    if enemy.floor == player.floor {
        if let Object { viewshed: Some(view), ai: Some(ai), .. } = enemy {
            if view.visible.contains(&player_pos) && enemy.floor == player.floor {
                ai.target = Some(0);
                ai.state = AIState::Chasing;
                ai.tgt_memory = 24;
                ai.tgt_heatmap.reset_to_single_node(&player_pos, 5);

                let mut dest: Point = pos;
                let distance = DistanceAlg::Pythagoras.distance2d(pos, player_pos);
                let targets = vec![map.index(player_pos.x, player_pos.y)];
                let dijkstra_map = DijkstraMap::new(90, 90, &targets, map, 1024.0);

                if let Some(destidx) = DijkstraMap::find_lowest_exit(&dijkstra_map, map.index(pos.x, pos.y), map) {
                    dest = if distance > 1.45 {
                        map.point_from_idx(destidx)
                    } else {
                        player_pos
                    };
                }
                if distance <= 1.45 {
                    enemy.try_attack(player, rng);
                } else if dest != pos { enemy.try_move(dest, map) }
            } else if ai.tgt_memory > 0 {
                ai.state = AIState::Hunting;
                ai.tgt_memory -= 1;
                ai.tgt_heatmap.spread(pos, map);
                let dest = ai.tgt_heatmap.get_closest_heat(map, pos);
                if dest != pos { enemy.try_move(dest, map) }
            } else {
                ai.target = None;
                ai.state = AIState::Idle;
            }
        }
        clear_ai_heatmap(enemy);
    }
}

fn clear_ai_heatmap(enemy: &mut Object) {
    if let Object { viewshed: Some(view), ai: Some(ai), ..} = enemy {
        ai.tgt_heatmap.clear_heat_area(&view.visible);
    }
}