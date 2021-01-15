use crate::prelude::*;

#[derive(Default)]
pub struct Object {
    pub name: Option<String>,
    pub pos: Option<Point>,
    pub render: Option<Render>
}

//Component Definitions:

pub struct Render {
    pub glyph: FontCharType,
    pub color: ColorPair
}
