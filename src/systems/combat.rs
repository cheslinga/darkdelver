use crate::prelude::*;

pub fn proc_all_wounds(objects: &mut Vec<Object>, logs: &mut LogBuffer, player_death: &mut bool) {
    let mut kill_list: Vec<usize> = Vec::new();
    let mut woundlist: InitList = InitList::new();

    for (i, obj) in objects.iter().enumerate() {
        woundlist.add_object(i, obj.initiative.unwrap_or(0));
    }
    woundlist.sort();

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
                } //total
                logs.update_logs(LogMessage::new()
                    .add_part(format!("{}", obj.name.as_ref().unwrap()), ColorPair::new(obj.render.as_ref().unwrap().color.fg, BLACK))
                    .add_part(format!("takes {} damage.", total), ColorPair::new(WHITE, BLACK))
                );
                health.wounds.clear();
            }
            //If it should be dead, make sure it gets killed at the end
            if health.current <= 0 {
                kill_list.push(id);
                logs.update_logs(LogMessage::new()
                    .add_part(format!("{}", obj.name.as_ref().unwrap()), ColorPair::new(obj.render.as_ref().unwrap().color.fg, BLACK))
                    .add_part("has been slain.", ColorPair::new(WHITE, BLACK))
                );

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
        let pos = objects[*id].pos.unwrap();
        let floor = objects[*id].floor;
        objects.push(make_corpse(pos, floor));
        objects.remove(*id);
    }
}