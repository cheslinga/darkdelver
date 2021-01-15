use crate::prelude::*;

pub struct State {
    pub world: World,
    pub refresh_con: bool
}
impl State {
    pub fn init() -> State {
        State {
            world: World::init(),
            refresh_con: true
        }
    }
}
impl GameState for State {
    fn tick(&mut self, con: &mut BTerm) {
        player_input(self, con);

        exec_all_systems(self);

        if self.refresh_con {
            con.cls();
            batch_all(self);
            render_draw_buffer(con).expect("Error rendering draw buffer to the console!");
            self.refresh_con = false;
        }
    }
}

pub struct World {
    pub rng: RandomNumberGenerator,
    pub objects: Vec<Object>,
    pub active_map: Map,
    pub last_map: Option<Map>,
    pub depth: i32,
    pub camera: Camera
}
impl World {
    pub fn init() -> World {
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
    process_fov(
        (&mut gs.world.objects, &mut gs.world.active_map)
    );
}
