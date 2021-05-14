use crate::prelude::*;
use std::collections::HashMap;

pub fn spawn_player(startpos: Point) -> Object {
    Object {
        name: Some("Player".to_string()),
        tag: Some(ActorTag::Player),
        pos: Some(startpos),
        floor: 1,
        render: Some(Render {
            glyph: 64,
            color: ColorPair::new(WHITE, BLACK),
            order: 255
        }),
        viewshed: Some(Viewshed {
            range: 5,
            visible: Vec::new(),
            refresh: true,
        }),
        block_tile: true,
        initiative: Some(12),

        health: Some(Health::new(48)),
        damage: Some(Damage::new(Damage::get_default_damage())),

        ..Default::default()
    }
}

pub fn make_beast(pos: Point, depth: i32) -> Object {
    Object {
        name: Some("Bloodthirsty Beast".to_string()),
        tag: Some(ActorTag::Enemy),
        pos: Some(pos),
        floor: depth,
        render: Some(Render {
            glyph: 98,
            color: ColorPair::new(RED, BLACK),
            order: 10
        }),
        viewshed: Some(Viewshed {
            range: 5,
            visible: Vec::new(),
            refresh: true,
        }),
        block_tile: true,
        initiative: Some(8),

        health: Some(Health::new(6)),
        damage: Some(Damage::new(Damage::get_default_damage())),

        ai: Some(AIClass::new()),

        ..Default::default()
    }
}

pub fn make_corpse(pos: Point, floor: i32) -> Object {
    Object {
        name: Some("A Corpse".to_string()),
        pos: Some(pos),
        floor,
        render: Some(Render {
            glyph: 37,
            color: ColorPair::new(MAROON, BLACK),
            order: 1
        }),

        ..Default::default()
    }
}


pub fn add_positional_info(init_obj: &mut Object, pos: Point, depth: i32) {
    init_obj.pos = Some(pos);
    init_obj.floor = depth;
}

pub fn get_enemy_spawn_table(depth: i32, num_enemies: i32, rng: &mut RandomNumberGenerator) -> Vec<Object> {
    let mut enemies: Vec<Object> = Vec::new();

    let conn = open_connection();

    //Some BS static table IDs correlated to depth
    let table_id = match depth {
        1|2|3 => 1,
        _ => 2
    };

    //Builds a list of enemy IDs to spawn
    let id_list= {
        let mut ids = Vec::new();

        let spawn_table = get_spawn_table_info(&conn, table_id).unwrap();
        let total_weight: u32 = {
            let mut t: u32 = 0;
            for e in spawn_table.iter() { t += e.weight as u32 }
            t
        };

        for i in 1..=num_enemies {
            let mut pivot = rng.range(0, total_weight);
            for entry in spawn_table.iter() {
                match pivot < entry.weight as u32 {
                    true => {
                        ids.push(entry.enemy_id);
                        break
                    }
                    false => { pivot -= entry.weight as u32 }
                }
            }
        }
        ids
    };

    //Iterate the list of IDs once, and populate a hashmap with enemy objects keyed by ID (reduces total number of DB calls)
    let mut obj_map: HashMap<u32, Object> = HashMap::new();
    for id in id_list.iter() {
        if !obj_map.contains_key(id) {
            let mut enemy_q = import_enemies_to_objects(&conn,
                                                        String::from("V_EnemiesFull"),
                                                        Some(format!("id = {}", id))
            ).expect("Failed to import enemy spawn table from the database.");
            obj_map.insert(*id, enemy_q[0].clone());
        }
    }

    //Iterate the list again, and fill up the objects vector
    for id in id_list.iter() {
        enemies.push(obj_map.get(id).unwrap().clone())
    }

    conn.close().expect("Connection to SQLite DB failed to close.");
    return enemies
}

pub fn get_starting_equip() -> Vec<Object> {
    let conn = open_connection();
    let mut items = import_items_to_objects(&conn,
                                            String::from("V_ItemsFull"),
                                            Some(format!("id IN (1,3)"))
    ).expect("Failed to import starting items from the database.");
    conn.close().expect("Connection to SQLite DB failed to close.");

    for obj in items.iter_mut() {
        obj.in_inventory = Some(InInventory{ owner_id: 0 })
    }
    return items
}

pub fn give_items(objects: &mut Vec<Object>, obj_id: usize, item_ids: Vec<i32>) {
    let ids_in = {
        let mut base_string = String::new();
        for id in item_ids.iter() {
            base_string.push_str(&*format!("{},", id))
        }
        base_string.pop();
        base_string
    };

    let conn = open_connection();
    let mut items = import_items_to_objects(&conn,
                                            String::from("V_ItemsFull"),
                                            Some(format!("id IN ({})", ids_in))
    ).expect("Failed to import starting items from the database.");
    conn.close().expect("Connection to SQLite DB failed to close.");

    for obj in items.iter_mut() { obj.in_inventory = Some(InInventory{ owner_id: obj_id }); }
    for obj in items.into_iter() { objects.push(obj) }
}