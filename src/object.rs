use crate::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Default,Serialize,Deserialize)]
pub struct Object {
    pub name: Option<String>,
    pub tag: Option<ActorTag>,
    pub pos: Option<Point>,
    pub render: Option<Render>,
    pub viewshed: Option<Viewshed>
}
impl Object {
    pub fn blank() -> Object {
        Object{..Default::default()}
    }
}

//Component Definitions:
#[derive(Serialize,Deserialize)]
pub enum ActorTag {
    None,
    Static,
    Player,
    Enemy
}
impl Default for ActorTag { fn default() -> Self { ActorTag::None } }

#[derive(Serialize,Deserialize)]
pub struct Render {
    pub glyph: FontCharType,
    pub color: ColorPair
}

#[derive(Serialize,Deserialize)]
pub struct Viewshed {
    pub range: i32,
    pub visible: Vec<Point>,
    pub refresh: bool
}
