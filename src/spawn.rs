use crate::prelude::*;

pub fn spawn_player(startpos: Point) -> Object {
    Object {
        name: Some("Player".to_string()),
        tag: Some(ActorTag::Player),
        pos: Some(startpos),
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

        health: Some(Health::new(24)),
        damage: Some(Damage::new(1,6)),

        ..Default::default()
    }
}

pub fn make_beast(pos: Point) -> Object {
    Object {
        name: Some("Bloodthirsty Beast".to_string()),
        tag: Some(ActorTag::Enemy),
        pos: Some(pos),
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

pub fn make_corpse(pos: Point) -> Object {
    Object {
        name: Some("A Corpse".to_string()),
        pos: Some(pos),
        render: Some(Render {
            glyph: 37,
            color: ColorPair::new(MAROON, BLACK),
            order: 1
        }),

        ..Default::default()
    }
}