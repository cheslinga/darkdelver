use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::slice::Iter;
use std::collections::HashSet;

#[derive(Default, Serialize, Deserialize)]
pub struct Object {
    pub name: Option<String>,
    pub tag: Option<ActorTag>,
    pub pos: Option<Point>,
    pub floor: i32,
    pub render: Option<Render>,
    pub player_mem: PlayerMemory,
    pub viewshed: Option<Viewshed>,
    pub block_tile: bool,
    pub initiative: Option<u8>,

    pub in_inventory: Option<InInventory>,
    pub equip_slot: Option<EquipSlot>,

    pub health: Option<Health>,
    pub damage: Option<Damage>,

    pub ai: Option<AIClass>,
    pub item_stats: Option<ItemStats>
}
impl Object {
    pub fn blank() -> Object {
        Object {
            floor: 1,
            block_tile: false,
            player_mem: PlayerMemory { seen: false, last_pos: None },
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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Render {
    pub glyph: FontCharType,
    pub color: ColorPair,
    pub order: u8
}
impl Render {
    pub fn nil_render() -> Render {
        Render {
            glyph: 0,
            color: ColorPair::new(WHITE,BLACK),
            order: 0
        }
    }
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

#[derive(Serialize, Deserialize)]
pub struct PlayerMemory {
    pub seen: bool,
    pub last_pos: Option<Point>
}
impl Default for PlayerMemory {
    fn default() -> Self { PlayerMemory { seen: false, last_pos: None } }
}

#[derive(Serialize,Deserialize)]
pub struct InInventory {
    pub owner_id: usize
}
#[derive(Clone,Copy,Serialize,Deserialize,Eq,PartialEq,Hash)]
pub enum EquipSlot {
    Head,Body,Arms,Legs,Feet,MainHand,OffHand,TwoHand,AnyHand,Ring1,Ring2
}
impl EquipSlot {
    pub fn get_all_slots() -> HashSet<EquipSlot> {
        return {
            let mut all_slots: HashSet<EquipSlot> = HashSet::new();
            for s in [EquipSlot::Arms, EquipSlot::Body, EquipSlot::Feet, EquipSlot::Head, EquipSlot::Legs, EquipSlot::OffHand, EquipSlot::MainHand,
                      EquipSlot::TwoHand, EquipSlot::AnyHand, EquipSlot::Ring1, EquipSlot::Ring2].iter()
            {
                all_slots.insert(*s);
            }
            all_slots
        }
    }
    pub fn match_db_string(db_string: String) -> Option<EquipSlot> {
        match db_string.as_str() {
            "MainHand" => Some(EquipSlot::MainHand),
            "OffHand" => Some(EquipSlot::OffHand),
            "AnyHand" => Some(EquipSlot::AnyHand),
            "2Hand" => Some(EquipSlot::TwoHand),
            _ => None
        }
    }
}