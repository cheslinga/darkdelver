use crate::prelude::*;

pub fn spawn_player(startpos: Point) -> Object {
    Object {
        name: Some("Player".to_string()),
        pos: Some(startpos),
        render: Some(Render {
            glyph: 64,
            color: ColorPair::new(WHITE,BLACK)
        }),
        ..Default::default()
    }
}
