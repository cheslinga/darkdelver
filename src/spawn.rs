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
        damage: Some(Damage::new(1,6)),

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
        damage: Some(Damage::new(1,3)),

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
    let mut items = Vec::new();

    items.push(
      Object {
          name: Some("Iron Longsword".to_string()),
          render: Some(Render {
              glyph: 24,
              color: ColorPair::new(GREY75, BLACK),
              order: 1
          }),
          in_inventory: Some(InInventory {
              owner_id: 0
          }),
          ..Default::default()
      }
    );
    items.push(
        Object {
            name: Some("Leather Jerkin".to_string()),
            render: Some(Render {
                glyph: 93,
                color: ColorPair::new(DARK_KHAKI, BLACK),
                order: 1
            }),
            in_inventory: Some(InInventory {
                owner_id: 0
            }),
            ..Default::default()
        }
    );
    items.push(
        Object {
            name: Some("Wooden Shield".to_string()),
            render: Some(Render {
                glyph: 79,
                color: ColorPair::new(BURLYWOOD, BLACK),
                order: 1
            }),
            in_inventory: Some(InInventory {
                owner_id: 0
            }),
            ..Default::default()
        }
    );
    items.push(
        Object {
            name: Some("Potion of Mending".to_string()),
            render: Some(Render {
                glyph: 173,
                color: ColorPair::new(LIME_GREEN, BLACK),
                order: 1
            }),
            in_inventory: Some(InInventory {
                owner_id: 0
            }),
            ..Default::default()
        }
    );

    return items
}


