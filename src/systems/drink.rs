use crate::prelude::*;

pub fn drink_object(objects: &mut Vec<Object>, source: usize, logs: &mut LogBuffer, rng: &mut RandomNumberGenerator) {
    let owner = objects[source].in_inventory.as_ref().unwrap().owner_id.clone();
    let name = objects[source].name.as_ref().unwrap_or(&format!("NIL")).clone();
    let color = objects[source].render.as_ref().unwrap_or(&Render::nil_render()).clone().color.fg;
    let effects = objects[source].item_stats.as_ref().unwrap_or(&ItemStats::blank_with_drop()).effects.clone();

    for effect in effects.into_iter() {
        match effect.etype {
            EffectType::HealSelf => {
                let (n, die_type) = {
                    let params = effect.params.unwrap();
                    (params[0], params[1])
                };
                let dice_roll = rng.roll_dice(n, die_type);

                if let Some(owner_health) = &mut objects[owner].health {
                    let amt_healed = owner_health.heal(dice_roll);
                    logs.update_logs(LogMessage::new()
                        .add_part("You drink the", ColorPair::new(WHITE,GREY10))
                        .add_part(&name, ColorPair::new(color, GREY10))
                        .add_part("healing you for", ColorPair::new(WHITE,GREY10))
                        .add_part(amt_healed.to_string(), ColorPair::new(GOLD,GREY10))
                        .add_part("points.", ColorPair::new(WHITE,GREY10))
                    );
                }

                objects[source].item_stats.as_mut().unwrap().effects_applied = true;
            }
            EffectType::DamageTgt => {

            }
            _ => {}
        }
    }
    if objects[source].item_stats.as_mut().unwrap().effects_applied {
        objects.remove(source);
    }
}