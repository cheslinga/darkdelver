use crate::prelude::*;

pub fn proc_all_wounds(objects: &mut Vec<Object>, player_death: &mut bool) {
    let mut kill_list: Vec<usize> = Vec::new();
    let mut woundlist: Vec<(usize, u8)> = Vec::new();

    for (i, obj) in objects.iter().enumerate() {
        woundlist.push((i, obj.initiative.unwrap_or(0)));
    }
    woundlist.sort_by_key(|a| Reverse(a.0));

    for sorted in woundlist.iter() {
        let id = sorted.0;
        let obj = &mut objects[id];

        if let Object { health: Some(health), .. } = obj {
            //Process each wound against the target's current health
            if health.wounds.len() > 0 {
                let mut total: i32 = 0;
                for wound in health.wounds.iter() {
                    health.current -= wound;
                    total += wound;
                }
                console::log(format!("{} takes {} damage.", obj.name.as_ref().unwrap(), total)); //TODO: Replace with actual on-screen logging
                health.wounds.clear();
            }
            //If it should be dead, make sure it gets killed at the end
            if health.current <= 0 {
                kill_list.push(id);
                console::log(format!("{} has been slain.", obj.name.as_ref().unwrap())); //TODO: Replace with actual on-screen logging

                if let Object { tag: Some(tag), .. } = obj {
                    if tag == &mut ActorTag::Player {
                        *player_death = true;
                    }
                }
            }
        }
    }

    //Kill anything that had 0 or less health
    for id in kill_list.iter() {
        objects.remove(*id);
    }
}