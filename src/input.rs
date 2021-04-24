use crate::prelude::*;

pub enum Actions {
    MoveUp,MoveDown,MoveLeft,MoveRight,
    MoveUpLeft,MoveUpRight,MoveDownLeft,MoveDownRight,
    TryPickUp,
    TryGoDown,
    Wait,
}

//Grabs the player's keypresses
pub fn player_input(gs: &mut State, con: &BTerm) {
    match gs.con_status {
        ContextStatus::InGame => ingame_input(gs, con),
        ContextStatus::InventoryOpen => inventory_input(gs, con, gs.inv.as_ref().unwrap().submenu.is_some()),
        ContextStatus::MainMenu |
        ContextStatus::PauseMenu => menu_input(gs, con),
    }
}

pub fn game_over_input(gs: &mut State, con: &BTerm) {
    if let Some(key) = con.key {
        match key {
            VirtualKeyCode::Return | VirtualKeyCode::R => {
                gs.con_status = ContextStatus::MainMenu;
                gs.menu = Some(Menu::main_menu());
                gs.turn_state = TurnState::Player;
                gs.refresh_con = true;
            },
            _ => {}
        }
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

            VirtualKeyCode::G
                => process_action(gs, Actions::TryPickUp),

            VirtualKeyCode::Slash
                => process_action(gs, Actions::TryGoDown),

            VirtualKeyCode::I => {
                gs.inv = Some(InventoryMenu::new(&gs.world.objects));
                gs.con_status = ContextStatus::InventoryOpen;
                gs.refresh_con = true;
            },

            //CHEATYFACE MODE
            /*
            VirtualKeyCode::Grave => {
                for obj in gs.world.objects.iter() {
                    println!("{} is at {},{}, with icon {}",
                             obj.name.as_ref().unwrap_or(&"NIL".to_string()),
                             obj.pos.as_ref().unwrap_or(&Point::zero()).x,
                             obj.pos.as_ref().unwrap_or(&Point::zero()).y,
                             to_char(obj.render.unwrap_or(Render { glyph: 1, color: ColorPair::new(BLACK, BLACK), order: 0 }).glyph as u8).to_string()
                    );
                }
            },
            */

            _ => {}
        }
    }
}

fn inventory_input(gs: &mut State, con: &BTerm, submenu: bool) {
    if !submenu {
        if let Some(key) = con.key {
            match key {
                VirtualKeyCode::Escape => { inv_clear(gs) },
                VirtualKeyCode::Return => { inv_trigger_select(gs) },
                VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => { inv_move_select(gs, UpDown::Up) },
                VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => { inv_move_select(gs, UpDown::Down) },
                VirtualKeyCode::A => { inv_trigger_set_select(0, gs) },
                VirtualKeyCode::B => { inv_trigger_set_select(1, gs) },
                VirtualKeyCode::C => { inv_trigger_set_select(2, gs) },
                VirtualKeyCode::D => { inv_trigger_set_select(3, gs) },
                VirtualKeyCode::E => { inv_trigger_set_select(4, gs) },
                VirtualKeyCode::F => { inv_trigger_set_select(5, gs) },
                VirtualKeyCode::G => { inv_trigger_set_select(6, gs) },
                VirtualKeyCode::H => { inv_trigger_set_select(7, gs) },
                VirtualKeyCode::I => { inv_trigger_set_select(8, gs) },
                VirtualKeyCode::J => { inv_trigger_set_select(9, gs) },
                VirtualKeyCode::K => { inv_trigger_set_select(10, gs) },
                VirtualKeyCode::L => { inv_trigger_set_select(11, gs) },
                VirtualKeyCode::M => { inv_trigger_set_select(12, gs) },
                VirtualKeyCode::N => { inv_trigger_set_select(13, gs) },
                VirtualKeyCode::O => { inv_trigger_set_select(14, gs) },
                VirtualKeyCode::P => { inv_trigger_set_select(15, gs) },
                VirtualKeyCode::Q => { inv_trigger_set_select(16, gs) },
                VirtualKeyCode::R => { inv_trigger_set_select(17, gs) },
                VirtualKeyCode::S => { inv_trigger_set_select(18, gs) },
                VirtualKeyCode::T => { inv_trigger_set_select(19, gs) },
                VirtualKeyCode::U => { inv_trigger_set_select(20, gs) },
                VirtualKeyCode::V => { inv_trigger_set_select(21, gs) },
                VirtualKeyCode::W => { inv_trigger_set_select(22, gs) },
                VirtualKeyCode::X => { inv_trigger_set_select(23, gs) },
                VirtualKeyCode::Y => { inv_trigger_set_select(24, gs) },
                VirtualKeyCode::Z => { inv_trigger_set_select(25, gs) },
                _ => {}
            }
        }
    }
    else {
        if let Some(key) = con.key {
            match key {
                VirtualKeyCode::Escape => { sm_clear(gs) },
                VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => { sm_move_select(gs, UpDown::Up) },
                VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => { sm_move_select(gs, UpDown::Down) },
                VirtualKeyCode::Return => { sm_trigger_select(gs) }
                _ => {}
            }
        }
    }
    gs.refresh_con = true;
}
fn inv_trigger_select(gs: &mut State) {
    let inv = gs.inv.as_mut().unwrap();
    let objs = &mut gs.world.objects;
    inv.process_selection(objs);
}
fn inv_trigger_set_select(selection: usize, gs: &mut State) {
    let inv = gs.inv.as_mut().unwrap();
    let objs = &mut gs.world.objects;

    if selection >= inv.items.len() { return }
    else { inv.selection = selection; }

    inv.process_selection(objs);
}
fn inv_move_select(gs: &mut State, updown: UpDown) {
    let inv = gs.inv.as_mut().unwrap();
    match updown {
        UpDown::Up => inv.move_selection_up(),
        UpDown::Down => inv.move_selection_down(),
    }
    gs.refresh_con = true;
}
fn sm_move_select(gs: &mut State, updown: UpDown) {
    let sm = gs.inv.as_mut().unwrap().submenu.as_mut().unwrap();
    match updown {
        UpDown::Up => sm.move_selection_up(),
        UpDown::Down => sm.move_selection_down()
    }
}
fn sm_trigger_select(gs: &mut State) {
    let sm = gs.inv.as_mut().unwrap().submenu.as_mut().unwrap();
    let objs = &mut gs.world.objects;
    let logs = &mut gs.logs;
    let rng = &mut gs.world.rng;

    sm.process_selection(objs, logs, rng);

    inv_clear(gs);
    gs.proc = true;
}
enum UpDown {Up,Down}

fn inv_clear(gs: &mut State) {
    gs.inv = None;
    gs.con_status = ContextStatus::InGame;
    gs.refresh_con = true;
}
fn sm_clear(gs: &mut State) {
    let inv = gs.inv.as_mut().unwrap();
    inv.submenu = None;
    gs.refresh_con = true;
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
                => { if gs.con_status == ContextStatus::PauseMenu { gs.con_status = ContextStatus::InGame } },
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

        Actions::TryPickUp => { try_pick_up(&mut gs.world.objects, 0, &mut gs.logs, true); true },

        Actions::TryGoDown => try_go_downstairs(gs)
    };
    gs.refresh_con = true;
    gs.proc = true;
    if actionresult { gs.passed = true; }
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
            if pos == dest && obj.floor == player[0].floor && tag == &mut ActorTag::Enemy {
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

//Attempts to walk down a downward staircase
fn try_go_downstairs(gs: &mut State) -> bool {
    let map = &gs.world.active_map;
    let player = &gs.world.objects[0];

    let pos = player.pos.unwrap();
    if map.tiles[map.index(pos.x, pos.y)] == TileClass::DownStair {
        gs.logs.update_logs(LogMessage::new()
            .add_part(format!("Descending to level {}...", gs.world.depth + 1), ColorPair::new(GREY13, WHITE))
        );
        gs.world.descend_to_next();
        return true
    } else {
        gs.logs.update_logs(LogMessage::new()
            .add_part("No stairs to descend!", ColorPair::new(GREY65, GREY10))
        );
        return false
    }
}
