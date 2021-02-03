use crate::prelude::*;

pub fn proc_all_wounds(objects: &mut Vec<Object>, turn_state: &mut TurnState) {
    let mut kill_list: Vec<usize> = Vec::new();

    for (i, obj) in objects.iter_mut().enumerate() {
        if let Object { health: Some(health), .. } = obj {
            //Process each wound against the target's current health
            if health.wounds.len() > 0 {
                for wound in health.wounds.iter() {
                    health.current -= wound;
                    console::log(format!("{} takes {} damage.", obj.name.as_ref().unwrap(), wound)); //TODO: Replace with actual on-screen logging
                }
                health.wounds.clear();
            }
            //If it should be dead, make sure it gets killed at the end
            if health.current <= 0 {
                kill_list.push(i);
                console::log(format!("{} has been slain.", obj.name.as_ref().unwrap())); //TODO: Replace with actual on-screen logging

                if let Object { tag: Some(tag), .. } = obj {
                    if tag == &mut ActorTag::Player {
                        *turn_state = TurnState::GameOver;
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