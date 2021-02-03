use crate::prelude::*;

pub enum Actions {
    MoveUp,MoveDown,MoveLeft,MoveRight,
    MoveUpLeft,MoveUpRight,MoveDownLeft,MoveDownRight,
    Wait,
}

//Grabs the player's keypresses
pub fn player_input(gs: &mut State, con: &BTerm) {
    match gs.con_status {
        ContextStatus::InGame => ingame_input(gs, con),
        ContextStatus::MainMenu |
        ContextStatus::PauseMenu |
        ContextStatus::GameOver => menu_input(gs, con),
    }
}

fn ingame_input(gs: &mut State, con: &BTerm) {
    if let Some(key) = con.key {
        match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H
                => process_action(gs, Actions::MoveLeft),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L
                => process_action(gs, Actions::MoveRight),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::J
                => process_action(gs, Actions::MoveUp),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::K
                => process_action(gs, Actions::MoveDown),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y
                => process_action(gs, Actions::MoveUpLeft),
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U
                => process_action(gs, Actions::MoveUpRight),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B
                => process_action(gs, Actions::MoveDownLeft),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N
                => process_action(gs, Actions::MoveDownRight),

            VirtualKeyCode::Numpad5 | VirtualKeyCode::Period
            => process_action(gs, Actions::Wait),

            VirtualKeyCode::Escape => {
                gs.menu = Some(Menu::pause_menu());
                gs.con_status = ContextStatus::PauseMenu;
                gs.refresh_con = true;
            },

            _ => {}
        }
    }
}

fn menu_input(gs: &mut State, con: &BTerm) {
    if let Some(key) = con.key {
        match key {
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::J
                => gs.menu.as_mut().unwrap().cycle_selection_up(),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::K
                => gs.menu.as_mut().unwrap().cycle_selection_down(),
            VirtualKeyCode::Return
                => gs.menu.as_mut().unwrap().process_selection(),
            VirtualKeyCode::Escape
                => gs.con_status = ContextStatus::InGame,
            _ => {}
        }
        gs.refresh_con = true;
    }
}

fn process_action(gs: &mut State, action: Actions) {
    let actionresult: bool = match action {
        Actions::Wait => true,

        Actions::MoveLeft => try_move_player(gs, DL_LEFT),
        Actions::MoveRight => try_move_player(gs, DL_RIGHT),
        Actions::MoveUp => try_move_player(gs, DL_UP),
        Actions::MoveDown => try_move_player(gs, DL_DOWN),

        Actions::MoveUpLeft => try_move_player(gs, DL_UP + DL_LEFT),
        Actions::MoveUpRight => try_move_player(gs, DL_UP + DL_RIGHT),
        Actions::MoveDownLeft => try_move_player(gs, DL_DOWN + DL_LEFT),
        Actions::MoveDownRight => try_move_player(gs, DL_DOWN + DL_RIGHT),
    };
    gs.refresh_con = true;
    gs.proc = true;
    if actionresult { gs.turn_state = TurnState::AI; }
}

//Attempts to move the player to another tile
fn try_move_player(gs: &mut State, delta: Point) -> bool {
    let map = &gs.world.active_map;
    let camera = &mut gs.world.camera;
    let player = &mut gs.world.objects[0];

    let mut dest = player.pos.unwrap() + delta;

    player.try_move(dest, map);
    camera.move_camera(player.pos.unwrap());

    return if player.pos.unwrap() == dest { true } else { try_attack_player(gs, &mut dest) }
}

//Attempts to attack something
fn try_attack_player(gs: &mut State, dest: &mut Point) -> bool {
    let (player, all) = gs.world.objects.split_at_mut(1);
    let mut target: Option<&mut Object> = None;

    for obj in all.iter_mut() {
        if let Object { pos: Some(pos), tag: Some(tag), health: Some(_), .. } = obj {
            if pos == dest && tag == &mut ActorTag::Enemy {
                target = Some(obj);
            }
        }
    }

    return if let Some(tgt) = target {
        player[0].try_attack(tgt, &mut gs.world.rng);
        true
    } else {
        false
    }
}
