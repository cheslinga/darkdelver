use crate::prelude::*;

pub fn spawn_player(startpos: Point) -> Object {
    Object {
        name: Some("Player".to_string()),
        tag: Some(ActorTag::Player),
        pos: Some(startpos),
        render: Some(Render {
            glyph: 64,
            color: ColorPair::new(WHITE,BLACK)
        }),
        viewshed: Some(Viewshed {
            range: 5,
            visible: Vec::new(),
            refresh: true
        }),
        ..Default::default()
    }
}
