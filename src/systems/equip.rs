use crate::prelude::*;
use std::collections::HashSet;
use std::ops::Neg;

pub fn equip_object(objects: &mut Vec<Object>, source: usize, logs: &mut LogBuffer) {
    let owner = objects[source].in_inventory.as_ref().unwrap().owner_id.clone();
    let slot = objects[source].equip_slot.as_ref().unwrap().clone();
    let name = objects[source].name.as_ref().unwrap_or(&format!("NIL")).clone();
    let color = objects[source].render.as_ref().unwrap_or(&Render::nil_render()).clone().color.fg;

    //Check all items in the owner's inventory, eliminate any slots that are used up
    let mut valid_slots: HashSet<EquipSlot> = EquipSlot::get_all_slots();
    {
        for obj in objects.iter() {
            if let Object { in_inventory: Some(inv), item_stats: Some(stats), equip_slot: Some(this_slot), .. } = &obj {
                if inv.owner_id == owner {
                    if stats.equipped {
                        if this_slot == &EquipSlot::TwoHand {
                            valid_slots.remove(&EquipSlot::MainHand);
                            valid_slots.remove(&EquipSlot::OffHand);
                            valid_slots.remove(&EquipSlot::TwoHand);
                        }
                        else if this_slot == &EquipSlot::MainHand || this_slot == &EquipSlot::OffHand || this_slot == &EquipSlot::AnyHand {
                            valid_slots.remove(&EquipSlot::TwoHand);
                            valid_slots.remove(this_slot);
                        }
                        else {
                            valid_slots.remove(this_slot);
                        }
                    }
                }
            }
        }
    }

    if !valid_slots.contains(&slot) {
        logs.update_logs(LogMessage::new()
            .add_part("You cannot equip the", ColorPair::new(WHITE,GREY10))
            .add_part(format!("{}.",name), ColorPair::new(color, GREY10))
        );
    }
    else {
        objects[source].item_stats.as_mut().unwrap().equipped = true;
        process_effect_modifiers(objects, source, false);
        logs.update_logs(LogMessage::new()
            .add_part("You equip the", ColorPair::new(WHITE,GREY10))
            .add_part(format!("{}.",name), ColorPair::new(color, GREY10))
        );
    }

}
pub fn unequip_object(objects: &mut Vec<Object>, source: usize, logs: &mut LogBuffer) {
    let name = objects[source].name.as_ref().unwrap_or(&format!("NIL")).clone();
    let color = objects[source].render.as_ref().unwrap_or(&Render::nil_render()).clone().color.fg;

    objects[source].item_stats.as_mut().unwrap().equipped = false;
    process_effect_modifiers(objects, source, true);
    logs.update_logs(LogMessage::new()
        .add_part("You unequip the", ColorPair::new(WHITE,GREY10))
        .add_part(format!("{}.",name), ColorPair::new(color, GREY10))
    );
}

pub fn process_effect_modifiers(objects: &mut Vec<Object>, item_id: usize, clean: bool) {
    let owner = objects[item_id].in_inventory.as_ref().unwrap().owner_id.clone();

    //Collect clones of each of the items effects
    let mut effects: Vec<ItemEffect> = Vec::new();
    {
        objects[item_id].item_stats.as_mut().unwrap().effects_applied = true;
        for effect in &objects[item_id].item_stats.as_ref().unwrap().effects {
            effects.push((effect.clone()));
        }
    }
    //Negate all values if the clean flag is passed in
    if clean {
        for effect in effects.iter_mut() {
            if effect.on_equip && effect.etype != EffectType::WeaponDamage {
                if let Some(params) = &mut effect.params {
                    for m in params.iter_mut() {
                        *m = m.neg();
                    }
                }
            }
        }
    }
    //Apply all effects to the owner
    {
        let actor_obj = &mut objects[owner];
        for effect in effects.iter() {
            match effect.etype {
                EffectType::HealSelf => {
                    if let Some(health) = &mut actor_obj.health {
                        health.current += effect.params.as_ref().unwrap()[0];
                    }
                }
                EffectType::DamageTgt => {}
                EffectType::WeaponDamage => {
                    if let Some(dmg) = &mut actor_obj.damage {
                        if clean {
                            let (dice,val) = Damage::get_default_damage();
                            dmg.dice = dice;
                            dmg.val = val;
                        }
                        else {
                            dmg.dice = effect.params.as_ref().unwrap()[0];
                            dmg.val = effect.params.as_ref().unwrap()[1];
                        }
                    }
                }
                EffectType::HealthUp => {
                    if let Some(health) = &mut actor_obj.health {
                        health.max += effect.params.as_ref().unwrap()[0];
                        health.current += effect.params.as_ref().unwrap()[0];
                    }
                }
                EffectType::AttackUp => {}
                EffectType::NIL => {}
            }
        }
    }
}