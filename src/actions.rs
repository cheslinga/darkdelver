use crate::prelude::*;

impl Object {
    //Attempts to move an object, modifying its position
    pub fn try_move(&mut self, dest: Point, map: &Map) {
        if self.pos.is_some() {
            if !map.walkable(dest.x, dest.y) {
                return
            }
            else {
                self.pos = Some(dest);
                if let Object { viewshed: Some(view), .. } = self {
                    view.refresh = true
                }
            }
        }
        else {
            console::log("ERROR: Entity attempted to move without positional component.")
        }
    }

    //Attempts to attack another entity
    pub fn try_attack(&mut self, target: &mut Object, rng: &mut RandomNumberGenerator) {
        if let Object { damage: Some(dmg), .. } = self {
            if let Object { health: Some(tgt_health), .. } = target {
                //TODO: Add to-hit rolls?
                let dmgval = dmg.roll(rng);
                tgt_health.wounds.push(dmgval);
            }
            else {
                console::log("ERROR: Attack was wrongfully attempted against a non-damageable entity.")
            }
        }
        else {
            console::log("ERROR: Entity attempted to attack without damage component.")
        }
    }
}

