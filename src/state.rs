use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq)]
pub enum ContextStatus{ InGame, MainMenu, PauseMenu }

pub struct State {
    pub world: World,
    pub menu: Option<Menu>,
    pub exit: bool,
    pub con_status: ContextStatus,
    pub refresh_con: bool
}
impl State {
    pub fn init() -> State {
        State {
            world: World::empty(),
            menu: Some(Menu::main_menu()),
            exit: false,
            con_status: ContextStatus::MainMenu,
            refresh_con: true
        }
    }
    fn handle_menu_actions(&mut self) {
        if let Some(selection) = self.menu.as_ref().unwrap().processed_selection {
            match selection {
                MenuSelection::NewGame => {
                    self.world = World::new_game();
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                },
                MenuSelection::SaveGame => {
                    export_world(&self.world);
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                },
                MenuSelection::LoadGame => {
                    load_world(self);
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                },
                MenuSelection::Quit => {
                    self.exit = true
                },
                MenuSelection::Continue => {
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                }
            }
        }
    }
}
impl GameState for State {
    fn tick(&mut self, con: &mut BTerm) {
        player_input(self, con);

        match self.con_status {
            ContextStatus::InGame => {
                exec_all_systems(self);

                if self.refresh_con {
                    con.cls();
                    batch_all(self);
                    render_draw_buffer(con).expect("Error rendering draw buffer to the console!");
                    self.refresh_con = false;
                }
            },
            ContextStatus::MainMenu | ContextStatus::PauseMenu => {
                if self.refresh_con {
                    con.cls();

                    match self.con_status {
                        ContextStatus::MainMenu => batch_main_menu(self.menu.as_ref().unwrap()),
                        ContextStatus::PauseMenu => batch_pause_menu(self.menu.as_ref().unwrap()),
                        _ => {}
                    }

                    render_draw_buffer(con).expect("Error rendering draw buffer to the console!");
                    self.refresh_con = false;
                }
                self.handle_menu_actions();
            }
        }

        if self.exit == true {con.quit()}
    }
}

#[derive(Serialize,Deserialize)]
pub struct World {
    pub rng: RandomNumberGenerator,
    pub objects: Vec<Object>,
    pub active_map: Map,
    pub last_map: Option<Map>,
    pub depth: i32,
    pub camera: Camera
}
impl World {
    pub fn empty() -> World {
        World {
            rng: RandomNumberGenerator::new(),
            objects: Vec::new(),
            active_map: Map::new(0,0),
            last_map: None,
            depth: 0,
            camera: Camera::new(Point::zero())
        }
    }
    pub fn new_game() -> World {
        let mut rng = RandomNumberGenerator::new();
        let mapgen = MapGenerator::random_rooms_build(60, 60, &mut rng);

        let player = spawn_player(mapgen.rooms[0].center());
        let startpos = player.pos.unwrap();

        let mut world = World {
            rng: rng,
            objects: Vec::new(),
            active_map: mapgen.map,
            last_map: None,
            depth: 1,
            camera: Camera::new(Point::new(startpos.x, startpos.y))
        };
        world.objects.push(player);
        return world;
    }
}

fn exec_all_systems(gs: &mut State) {
    process_fov(&mut gs.world.objects, &mut gs.world.active_map);
}
