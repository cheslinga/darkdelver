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
                if total > 0 { health.reset_regen() }
                let (name, verb) = {
                    if id == 0 {
                        (String::from("You"), String::from("take"))
                    }
                    else {
                        (obj.name.as_ref().unwrap().clone(), String::from("takes"))
                    }
                };
                logs.update_logs(LogMessage::new()
                    .add_part(name, ColorPair::new(obj.render.as_ref().unwrap().color.fg, GREY10))
                    .add_part(format!("{} {} damage.", verb, total), ColorPair::new(WHITE, GREY10))
                );
                health.wounds.clear();
            }
            //If it should be dead, make sure it gets killed at the end
            if health.current <= 0 {
                kill_list.push(id);

                let (name, verb) = {
                    if id == 0 {
                        (String::from("You"), String::from("have"))
                    }
                    else {
                        (obj.name.as_ref().unwrap().clone(), String::from("has"))
                    }
                };
                logs.update_logs(LogMessage::new()
                    .add_part(name, ColorPair::new(obj.render.as_ref().unwrap().color.fg, GREY10))
                    .add_part(format!("{} been slain.", verb), ColorPair::new(WHITE, GREY10))
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

pub fn proc_regen(objects: &mut Vec<Object>) {
    for obj in objects.iter_mut() {
        if let Object { health: Some(health), .. } = obj {
            health.check_regen()
        }
    }
}