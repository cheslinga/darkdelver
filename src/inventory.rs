use crate::prelude::*;
use std::cmp::min;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct ItemStats {
    pub usages: Vec<ItemUsage>,
    pub effects: Vec<ItemEffect>,
    pub equipped: bool,
    pub effects_applied: bool
}
impl ItemStats {
    pub fn new(usages: Vec<ItemUsage>, effects: Vec<ItemEffect>) -> ItemStats { ItemStats { usages, effects, equipped: false, effects_applied: false } }
    pub fn blank() -> ItemStats { ItemStats { usages: vec![], effects: vec![], equipped: false, effects_applied: false } }
    pub fn blank_with_drop() -> ItemStats { ItemStats { usages: vec![ItemUsage::Drop], effects: vec![ItemEffect::nil()], equipped: false, effects_applied: false } }
}
impl Clone for ItemStats {
    fn clone(&self) -> Self { ItemStats { usages: self.usages.to_vec(), effects: self.effects.to_vec(), equipped: self.equipped, effects_applied: self.effects_applied } }
}

#[derive(Clone,Copy,PartialEq,Serialize,Deserialize)]
pub enum ItemUsage {
    Drop,
    Throw,
    Equip,
    Drink,
    Activate
}
impl ItemUsage {
    pub fn get_name(&self, is_equipped: bool) -> String {
        return match self {
            ItemUsage::Drop => "Drop",
            ItemUsage::Throw => "Throw",
            ItemUsage::Equip => {
                if is_equipped { "Unequip" }
                else { "Equip" }
            },
            ItemUsage::Drink => "Drink",
            ItemUsage::Activate => "Activate",
        }.to_string()
    }
    pub fn get_letter(&self) -> char {
        return match self {
            ItemUsage::Drop => 'd',
            ItemUsage::Throw => 't',
            ItemUsage::Equip => 'e',
            ItemUsage::Drink => 'q',
            ItemUsage::Activate => 'a',
        }
    }
}

#[derive(Clone,Copy,PartialEq,Serialize,Deserialize)]
pub enum EffectType {
    NIL,
    //Targeted effect variants
    HealSelf, DamageTgt,
    //On Equip variants
    WeaponDamage, AttackUp, HealthUp
}
impl SqlStringImport for EffectType {
    fn match_db_string(db_string: String) -> Option<EffectType> {
        match db_string.as_str() {
            "WeaponDmg" => Some(EffectType::WeaponDamage),
            "HealSelf" => Some(EffectType::HealSelf),
            "DamageTgt" => Some(EffectType::DamageTgt),
            "AttackUp" => Some(EffectType::AttackUp),
            "HealthUp" => Some(EffectType::HealthUp),
            _ => None
        }
    }
}
#[derive(Clone,Serialize,Deserialize)]
pub struct ItemEffect {
    pub etype: EffectType,
    pub params: Option<Vec<i32>>,
    pub on_equip: bool
}
impl ItemEffect {
    pub fn nil() -> ItemEffect { ItemEffect::default() }
}
impl Default for ItemEffect {
    fn default() -> Self { ItemEffect { etype: EffectType::NIL, params: None, on_equip: false } }
}


pub struct InventoryMenu {
    pub submenu: Option<InventorySubMenu>,
    pub items: Vec<ItemInfo>,
    pub selection: usize
}
pub struct InventorySubMenu {
    pub info: ItemInfo,
    pub opts: Vec<ItemUsage>,
    pub selection: usize
}
#[derive(Clone)]
pub struct ItemInfo {
    pub obj_id: usize,
    pub name: String,
    pub render: Render,
    pub stats: ItemStats
}

impl InventoryMenu {
    pub fn new(objects: &Vec<Object>) -> InventoryMenu {
        let mut menu = InventoryMenu {..Default::default()};
        menu.populate_items(objects);
        return menu
    }
    pub fn populate_items(&mut self, objects: &Vec<Object>) {
        for (i, obj) in objects.iter().enumerate() {
            if let Some(inv) = &obj.in_inventory {
                if inv.owner_id == 0 {
                    let info = ItemInfo {
                        obj_id: i,
                        name: obj.name.as_ref().unwrap().to_owned(),
                        render: *&obj.render.unwrap(),
                        stats: obj.item_stats.as_ref().unwrap().clone()
                    };
                    self.items.push(info);
                }
            }
        }
    }
    pub fn process_selection(&mut self) {
        return if self.items.len() <= 0 {}
        else { self.submenu = Some(InventorySubMenu::new(self.items[self.selection].clone())); }
    }
    pub fn move_selection_up(&mut self) {
        if self.items.len() <= 0 { return }
        if self.selection as i16 - 1 < 0 { self.selection = self.items.len() - 1 }
        else { self.selection -= 1 }
    }
    pub fn move_selection_down(&mut self) {
        if self.items.len() <= 0 { return }
        if self.selection + 1 >= self.items.len() { self.selection = 0 }
        else { self.selection += 1 }
    }
}
impl Default for InventoryMenu {
    fn default() -> InventoryMenu {
        InventoryMenu {
            submenu: None,
            items: Vec::new(),
            selection: 0
        }
    }
}

impl InventorySubMenu {
    pub fn new(info: ItemInfo) -> InventorySubMenu {
        let opts = info.stats.usages.to_vec();
        InventorySubMenu {
            info,
            opts,
            selection: 0
        }
    }

    pub fn move_selection_up(&mut self) {
        if self.opts.len() <= 0 { return }
        if self.selection as i16 - 1 < 0 { self.selection = self.opts.len() - 1 }
        else { self.selection -= 1 }
    }
    pub fn move_selection_down(&mut self) {
        if self.opts.len() <= 0 { return }
        if self.selection + 1 >= self.opts.len() { self.selection = 0 }
        else { self.selection += 1 }
    }
    pub fn process_selection(&mut self, objects: &mut Vec<Object>, logs: &mut LogBuffer, rng: &mut RandomNumberGenerator, pass_turn: &mut bool) {
        match self.opts[self.selection] {
            ItemUsage::Drop => {
                drop_item(objects, self.info.obj_id, logs);
                logs.update_logs(LogMessage::new()
                    .add_part("You have dropped", ColorPair::new(WHITE,GREY10))
                    .add_part(format!("{}.", &self.info.name), ColorPair::new(self.info.render.color.fg,GREY10))
                );
            },
            ItemUsage::Throw => {}
            ItemUsage::Equip => {
                if objects[self.info.obj_id].item_stats.as_ref().unwrap().equipped {
                    unequip_object(objects, self.info.obj_id, logs);
                }
                else {
                    equip_object(objects, self.info.obj_id, logs);
                }
                *pass_turn = true;
            }
            ItemUsage::Drink => {
                drink_object(objects, self.info.obj_id, logs, rng);
                *pass_turn = true;
            }
            ItemUsage::Activate => {}
        }
    }
}

//Item interaction functions
pub fn try_pick_up(objects: &mut Vec<Object>, source_obj: usize, logs: &mut LogBuffer, log_msg: bool) {
    let try_pos = objects[source_obj].pos.as_ref().unwrap_or(&Point::zero()).clone();
    let pickup_list = {
        let mut vec = Vec::new();
        for (i, o) in objects.iter().enumerate() {
            if let Some(pos) = o.pos { if pos == try_pos && o.item_stats.is_some() { vec.push(i) } }
        }
        vec
    };

    if pickup_list.len() == 1 {
        add_item_to_inventory(objects, source_obj, pickup_list[0], logs, log_msg);
    }
}

pub fn add_item_to_inventory(objects: &mut Vec<Object>, source_obj: usize, item: usize, logs: &mut LogBuffer, log_msg: bool) {
    let item = &mut objects[item];
    item.in_inventory = Some(InInventory { owner_id: source_obj });
    item.pos = None;

    if log_msg {
        let item_name = item.name.as_ref().unwrap_or(&format!("NIL")).clone();
        let item_colour = ColorPair::new(item.render.as_ref().unwrap_or(&Render::nil_render()).color.fg, GREY10);
        let (name, verb) = {
            if source_obj == 0 {
                (String::from("You"), String::from("pick"))
            }
            else {
                (if let Some(name) = objects[source_obj].name.clone() { name } else { String::from("Something") }, String::from("picks"))
            }
        };

        let owner_colour = ColorPair::new
            (objects[source_obj].render
                .unwrap_or(Render { glyph: 0, color: ColorPair::new(WHITE, BLACK), order: 0 })
                .color.fg,
             GREY10);

        logs.update_logs(LogMessage::new()
            .add_part(name, owner_colour)
            .add_part(verb, ColorPair::new(WHITE, GREY10))
            .add_part(String::from("up"), ColorPair::new(WHITE, GREY10))
            .add_part(format!("{}.", item_name), item_colour)
        );
    }
}

pub fn drop_item(objects: &mut Vec<Object>, item_id: usize, logs: &mut LogBuffer) {
    let owner = objects[item_id].in_inventory.as_ref().unwrap().owner_id.clone();
    let drop_pos = objects[owner].pos.clone();
    let floor = objects[owner].floor.clone();

    if drop_pos.is_some() {
        if objects[item_id].item_stats.as_mut().unwrap().equipped { unequip_object(objects, item_id, logs) }
        let item = &mut objects[item_id];
        item.in_inventory = None;
        item.pos = drop_pos;
        item.floor = floor;
    }
    else {
        console::log("Could not drop this item as no position to drop exists!");
    }
}



//Inventory menu rendering
pub fn batch_inventory_menu(menu: &mut InventoryMenu, objects: &Vec<Object>) {
    let mut uibatch = DrawBatch::new();
    let mut textbatch = DrawBatch::new();
    uibatch.target(OBJ_LAYER);
    textbatch.target(TXT_LAYER);

    let menubox = Rect::with_size(2, 2, CONSOLE_W - UI_CUTOFF.x - 4, min(menu.items.len() as i32 + 1, 18));
    uibatch.draw_double_box(menubox, ColorPair::new(GREY75, BLACK));
    textbatch.print(Point::new(8, 2), "Inventory");
    textbatch.print_color(Point::new(8, 2 + menubox.height()), "ESC to close", ColorPair::new(GOLD4, BLACK));

    let mut y = menubox.y1 + 1;
    let mut ofs: u16 = 0;
    for (i, item) in menu.items.iter().enumerate() {
        let line_color = match i == menu.selection {
            false => {ColorPair::new(WHITE,BLACK)},
            true => {ColorPair::new(BLACK,YELLOW)}
        };
        if y < menubox.y2 {
            uibatch.set(Point::new(menubox.x1 + 5, y), *&item.render.color, item.render.glyph);

            //Sets the letter to display next to the item
            uibatch.set(Point::new(menubox.x1 + 1, y), ColorPair::new(WHITE, BLACK), 40);
            uibatch.set(Point::new(menubox.x1 + 2, y), ColorPair::new(GOLD2, BLACK), 97 + ofs);
            uibatch.set(Point::new(menubox.x1 + 3, y), ColorPair::new(WHITE, BLACK), 41);

            textbatch.print_color(Point::new(menubox.x1 * 2 + 14, y), &item.name, line_color);
            y += 1;
            ofs += 1;
        }
    }

    if let Some(sub) = &mut menu.submenu {
        let smbox = Rect::with_size(CONSOLE_W - UI_CUTOFF.x - 25, 2, 24, 12);
        uibatch.draw_double_box(smbox, ColorPair::new(GREY75, BLACK));
        textbatch.print(Point::new((smbox.x1 + 1) * 2, smbox.y1 + 1), &sub.info.name);

        let mut ypos = smbox.y1 + 3;
        for (i, act) in sub.opts.iter().enumerate() {
            let select_color = match i == sub.selection {
                false => {ColorPair::new(WHITE,BLACK)},
                true => {ColorPair::new(BLACK,YELLOW)}
            };
            let equipped =
                if let Some(stats) = &objects[sub.info.obj_id].item_stats { stats.equipped }
                else { false };
            textbatch.print_color(Point::new((smbox.x1 + 1) * 2, ypos), act.get_name(equipped), select_color);
            ypos += 1;
        }
    }

    uibatch.submit(5100).expect("Failed to batch inventory menu draw");
    textbatch.submit(16000).expect("Failed to batch inventory menu draw");
}