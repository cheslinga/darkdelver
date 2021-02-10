use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq)]
pub enum TurnState { Player, AI, GameOver }

#[derive(PartialEq)]
pub enum ContextStatus{ InGame, MainMenu, PauseMenu }

pub struct State {
    pub world: World,
    pub turn_state: TurnState,
    pub menu: Option<Menu>,
    pub proc: bool,
    pub passed: bool,
    pub gameover: bool,
    pub exit: bool,
    pub con_status: ContextStatus,
    pub refresh_con: bool,
    pub logs: LogBuffer
}
impl State {
    pub fn init() -> State {
        State {
            world: World::empty(),
            turn_state: TurnState::Player,
            menu: Some(Menu::main_menu()),
            proc: true,
            passed: false,
            gameover: false,
            exit: false,
            con_status: ContextStatus::MainMenu,
            refresh_con: true,
            logs: LogBuffer::new()
        }
    }
    fn handle_menu_actions(&mut self) {
        if let Some(selection) = self.menu.as_ref().unwrap().processed_selection {
            match selection {
                MenuSelection::NewGame => {
                    self.world = World::new_game();
                    self.logs.clear();
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                    self.proc = true;

                    let welcome_msg = LogMessage::new()
                        .add_part("Your adventure begins now. ", ColorPair::new(WHITE,BLACK))
                        .add_part("Prepare to die...", ColorPair::new(RED, BLACK));
                    self.logs.push(welcome_msg);
                },
                MenuSelection::SaveGame => {
                    export_world(&self.world);
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                },
                MenuSelection::LoadGame => {
                    load_world(self);
                    self.logs.clear();
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                },
                MenuSelection::Quit => {
                    self.exit = true
                },
                MenuSelection::Continue => {
                    self.con_status = ContextStatus::InGame;
                    self.refresh_con = true;
                },
            }
        }
    }
}
impl GameState for State {
    fn tick(&mut self, con: &mut BTerm) {
        //Only take player input if it's the player's turn
        if self.turn_state == TurnState::Player { player_input(self, con) }
        else if self.turn_state == TurnState::GameOver { game_over_input(self, con) }

        match self.con_status {
            //If the game is in it's normal running state
            ContextStatus::InGame => {
                //Run all systems
                exec_all_systems(self);

                //Redraw to the console if it needs to be refreshed
                if self.refresh_con {
                    con.cls();
                    batch_all(&self.world.active_map, &self.world.camera, &self.world.objects, &self.logs, self.world.depth);
                    render_draw_buffer(con).expect("Error rendering draw buffer to the console!");
                    self.refresh_con = false;
                }
            },
            //If the game is in a menu of some sort
            ContextStatus::MainMenu | ContextStatus::PauseMenu => {
                //Redraw if necessary
                if self.refresh_con {
                    con.cls();
                    //Draw a different menu based on which menu is open at the moment
                    match self.con_status {
                        ContextStatus::MainMenu => batch_main_menu(self.menu.as_ref().unwrap()),
                        ContextStatus::PauseMenu => batch_pause_menu(self.menu.as_ref().unwrap()),
                        _ => {}
                    }
                    render_draw_buffer(con).expect("Error rendering draw buffer to the console!");
                    self.refresh_con = false;
                }
                //If any menu actions are ready to run, run them
                self.handle_menu_actions();
            },
        }

        //Close the game if the player chooses to exit
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
    pub camera: Camera,
}
impl World {
    pub fn empty() -> World {
        World {
            rng: RandomNumberGenerator::new(),
            objects: Vec::new(),
            active_map: Map::new(0,0),
            last_map: None,
            depth: 0,
            camera: Camera::new(Point::zero()),
        }
    }
    pub fn new_game() -> World {
        let mut rng = RandomNumberGenerator::new();
        let mapgen = MapGenerator::random_rooms_build(60, 60, &mut rng);

        let player = spawn_player(mapgen.rooms[0].center());
        let startpos = player.pos.unwrap();

        let mut world = World {
            rng,
            objects: Vec::new(),
            active_map: mapgen.map,
            last_map: None,
            depth: 1,
            camera: Camera::new(startpos),
        };

        world.objects.push(player);

        for room in mapgen.rooms.iter().skip(1) {
            world.objects.push(make_beast(room.center(), 1))
        }

        return world;
    }
    pub fn descend_to_next(&mut self) {
        //Copy the old map to the last_map member
        self.last_map = Some(Map::from_copy(&self.active_map));
        self.depth += 1;
        self.objects[0].floor = self.depth;

        //Set up a new map
        let mapgen = MapGenerator::random_rooms_build(60, 60, &mut self.rng);
        self.objects[0].pos = Some(mapgen.rooms[0].center());
        self.objects[0].viewshed.as_mut().unwrap().refresh = true;
        self.camera = Camera::new(mapgen.rooms[0].center());
        self.active_map = mapgen.map;

        for room in mapgen.rooms.iter().skip(1) {
            self.objects.push(make_beast(room.center(), self.depth))
        }

        //Clean up any objects that are 2 floors above
        let mut removelist: Vec<usize> = Vec::new();
        for (i, obj) in self.objects.iter().enumerate() {
            if obj.floor < self.depth - 1 {
                removelist.push(i);
            }
        }
        for i in removelist.iter() {
            self.objects.remove(*i);
        }
    }
}

fn exec_all_systems(gs: &mut State) {
    if gs.proc {
        process_fov(&mut gs.world.objects, &mut gs.world.active_map);
        update_blocked_tiles(&gs.world.objects, &mut gs.world.active_map, gs.world.depth);
        proc_all_wounds(&mut gs.world.objects, &mut gs.logs, &mut gs.gameover);

        //Check if the player's turn was passed
        if gs.passed {
            gs.turn_state = TurnState::AI;
            gs.passed = false;
        }

        //Run any stuff for the AI if it's the AI's turn
        if gs.turn_state == TurnState::AI {
            process_ai(&mut gs.world.objects, &mut gs.world.active_map, gs.world.depth, &mut gs.world.rng);
            update_blocked_tiles(&gs.world.objects, &mut gs.world.active_map, gs.world.depth);
            proc_all_wounds(&mut gs.world.objects, &mut gs.logs, &mut gs.gameover);
            gs.turn_state = TurnState::Player;
        }

        update_player_memory(&mut gs.world.objects);

        //Set the turn state on a game over event.
        if gs.gameover {
            gs.logs.update_logs(LogMessage::new()
                .add_part("Press", ColorPair::new(WHITE, BLACK))
                .add_part("Enter", ColorPair::new(LIME_GREEN, BLACK))
                .add_part("or", ColorPair::new(WHITE, BLACK))
                .add_part("R", ColorPair::new(LIME_GREEN, BLACK))
                .add_part("to return to the main menu.", ColorPair::new(WHITE, BLACK))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part(format!("You have perished on level {}.", gs.world.depth), ColorPair::new(BLACK, RED))
            );
            gs.turn_state = TurnState::GameOver;
            gs.gameover = false;
        }

        gs.proc = false;
    }
}
