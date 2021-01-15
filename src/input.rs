use crate::prelude::*;

pub enum Actions {
    MoveUp, MoveDown, MoveLeft, MoveRight,
    MoveUpLeft, MoveUpRight, MoveDownLeft, MoveDownRight
}

//Grabs the player's keypresses
pub fn player_input(gs: &mut State, con: &BTerm) {
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
            _ => {}
        }
    }
}

fn process_action(gs: &mut State, action: Actions) {
    match action {
        Actions::MoveLeft => try_move(gs, Point::new(-1, 0)),
        Actions::MoveRight => try_move(gs, Point::new(1, 0)),
        Actions::MoveUp => try_move(gs, Point::new(0, -1)),
        Actions::MoveDown => try_move(gs, Point::new(0, 1)),

        Actions::MoveUpLeft => try_move(gs, Point::new(-1, -1)),
        Actions::MoveUpRight => try_move(gs, Point::new(1, -1)),
        Actions::MoveDownLeft => try_move(gs, Point::new(-1, 1)),
        Actions::MoveDownRight => try_move(gs, Point::new(1, 1)),

        _ => {}
    }
    gs.refresh_con = true;
}

//Attempts to move to a (hopefully) walkable tile.
fn try_move(gs: &mut State, delta: Point) {
    let map = &gs.world.active_map;
    let camera = &mut gs.world.camera;
    let mut player = &mut gs.world.objects[0];

    let dest = player.pos.unwrap() + delta;
    if map.walkable(dest.x, dest.y) {
        player.pos = Some(dest);
        camera.move_camera(dest);
        player.viewshed.as_mut().unwrap().refresh = true;
    }
}
