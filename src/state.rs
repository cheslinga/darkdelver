use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq)]
pub enum TurnState { Player, AI, GameOver }

#[derive(PartialEq)]
pub enum ContextStatus{ InGame, InventoryOpen, MainMenu, PauseMenu }

pub struct MouseLocation {
    current: Point,
    prev: Point
}
impl MouseLocation {
    pub fn new() -> MouseLocation { MouseLocation { current: Point::zero(), prev: Point::zero() } }
    pub fn has_changed(&self) -> bool { return self.current != self.prev }
    pub fn get_pos(&mut self, con: &mut BTerm) {
        let ap = con.active_console;
        con.set_active_console(OBJ_LAYER);

        self.prev = self.current;
        self.current = con.mouse_point();

        con.set_active_console(ap);
    }
}

pub struct State {
    pub world: World,
    pub turn_state: TurnState,
    pub menu: Option<Menu>,
    pub inv: Option<InventoryMenu>,
    pub proc: bool,
    pub passed: bool,
    pub gameover: bool,
    pub exit: bool,
    pub con_status: ContextStatus,
    pub refresh_con: bool,
    pub logs: LogBuffer,
    pub mouse_pos: MouseLocation
}
impl State {
    pub fn init() -> State {
        State {
            world: World::empty(),
            turn_state: TurnState::Player,
            menu: Some(Menu::main_menu()),
            inv: None,
            proc: true,
            passed: false,
            gameover: false,
            exit: false,
            con_status: ContextStatus::MainMenu,
            refresh_con: true,
            logs: LogBuffer::new(),
            mouse_pos: MouseLocation::new()
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

                    self.logs.update_logs(LogMessage::new()
                        .add_part("Your adventure begins now. ", ColorPair::new(WHITE,GREY10))
                        .add_part("Prepare to die...", ColorPair::new(RED, GREY10))
                    );
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

        self.mouse_pos.get_pos(con);
        if self.mouse_pos.has_changed() { self.refresh_con = true }

        match self.con_status {
            //If the game is in it's normal running state
            ContextStatus::InGame | ContextStatus::InventoryOpen => {
                //Run all systems
                exec_all_systems(self);

                //Redraw to the console if it needs to be refreshed
                if self.refresh_con {
                    con.cls();
                    batch_all(&self.world.active_map, &self.world.camera, &self.world.objects, &self.logs, self.world.depth, self.mouse_pos.current);
                    if self.con_status == ContextStatus::InventoryOpen { batch_inventory_menu(self.inv.as_mut().unwrap(), &self.world.objects); }
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
        let num_rooms = mapgen.rooms.len();

        let startpos = mapgen.rooms[0].center();

        let mut world = World {
            rng,
            objects: Vec::new(),
            active_map: mapgen.map,
            last_map: None,
            depth: 1,
            camera: Camera::new(startpos),
        };

        //Spawn the player object
        let player = spawn_player(startpos);
        world.objects.insert(0, player);

        //Spawn starting equipment in the player's inventory
        let start_equip: Vec<Object> = get_starting_equip();
        for item in start_equip.into_iter() {
            world.objects.push(item);
        }

        //Spawn an enemy in the center of each room
        let enemy_spawns = get_enemy_spawn_table(1, num_rooms as i32 - 1, &mut world.rng);
        for (i, room) in mapgen.rooms.iter().enumerate().skip(1) {
            //This is temporary code since I've only got one enemy returning through this so far.
            let mut obj = enemy_spawns[i-1].clone();
            add_positional_info(&mut obj, room.center(), 1);
            world.objects.push(obj)
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

        let enemy_spawns = get_enemy_spawn_table(self.depth, mapgen.rooms.len() as i32 - 1, &mut self.rng);
        for (i, room) in mapgen.rooms.iter().enumerate().skip(1) {
            //Same as above. Will probably make a function out of it later when I have more enemies made :P
            let mut obj = enemy_spawns[i-1].clone();
            add_positional_info(&mut obj, room.center(), self.depth);
            self.objects.push(obj)
        }

        //Clean up any objects that are 2 floors above, but are not in any inventory
        let mut removelist: Vec<usize> = Vec::new();
        for (i, obj) in self.objects.iter().enumerate() {
            if obj.floor < self.depth - 1 && obj.in_inventory.is_none() {
                removelist.push(i);
            }
        }
        //Second pass for item inventory ownership; delete items on things that are to be deleted
        for (i, obj) in self.objects.iter().enumerate() {
            if let Some(inv) = &obj.in_inventory {
                if removelist.contains(&inv.owner_id) {
                    removelist.push(i);
                }
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
            process_fov(&mut gs.world.objects, &mut gs.world.active_map);
            proc_regen(&mut gs.world.objects);
        }

        //Run any stuff for the AI if it's the AI's turn
        if gs.turn_state == TurnState::AI {
            process_ai(&mut gs.world.objects, &mut gs.world.active_map, gs.world.depth, &mut gs.world.rng);
            process_fov(&mut gs.world.objects, &mut gs.world.active_map);
            proc_all_wounds(&mut gs.world.objects, &mut gs.logs, &mut gs.gameover);
            gs.turn_state = TurnState::Player;
        }

        update_player_memory(&mut gs.world.objects);

        //Set the turn state on a game over event.
        if gs.gameover {
            gs.logs.update_logs(LogMessage::new()
                .add_part("Press", ColorPair::new(WHITE, GREY10))
                .add_part("Enter", ColorPair::new(LIME_GREEN, GREY10))
                .add_part("or", ColorPair::new(WHITE, GREY10))
                .add_part("R", ColorPair::new(LIME_GREEN, GREY10))
                .add_part("to return to the main menu.", ColorPair::new(WHITE, GREY10))
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
