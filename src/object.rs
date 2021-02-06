use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Object {
    pub name: Option<String>,
    pub tag: Option<ActorTag>,
    pub pos: Option<Point>,
    pub render: Option<Render>,
    pub viewshed: Option<Viewshed>,
    pub block_tile: bool,
    pub initiative: Option<u8>,

    pub health: Option<Health>,
    pub damage: Option<Damage>
}
impl Object {
    pub fn blank() -> Object {
        Object {
            block_tile: false,
            ..Default::default()
        }
    }
}

//Component Definitions:
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ActorTag {
    None,
    Static,
    Player,
    Enemy,
}
impl Default for ActorTag {
    fn default() -> Self {
        ActorTag::None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Render {
    pub glyph: FontCharType,
    pub color: ColorPair,
    pub order: u8
}

#[derive(Serialize, Deserialize)]
pub struct Viewshed {
    pub range: i32,
    pub visible: Vec<Point>,
    pub refresh: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub max: i32,
    pub current: i32,
    pub wounds: Vec<i32>
}
impl Health {
    pub fn new(max: i32) -> Health {
        Health { max, current: max, wounds: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Damage {
    pub dice: i32,
    pub val: i32,
    pub modifiers: Vec<i32>
}
impl Damage {
    pub fn new(dice: i32, val: i32) -> Damage {
        Damage { dice, val, modifiers: Vec::new() }
    }
    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> i32 {
        let mut dmg: i32 = 0;
        dmg += rng.roll_dice(self.dice,self.val);

        if self.modifiers.len() > 0 {
            for m in self.modifiers.iter() {
                dmg += m;
            }
        }

        return dmg
    }
}