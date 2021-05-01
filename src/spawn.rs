use crate::prelude::*;

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

pub fn get_starting_equip() -> Vec<Object> {
    let conn = open_connection();
    let mut items = import_items_to_objects(&conn, String::from("StartingEq_TEST")).unwrap();
    conn.close().expect("Connection to SQLite DB failed to close.");

    for obj in items.iter_mut() {
        obj.in_inventory = Some(InInventory{ owner_id: 0 })
    }
    return items
}
