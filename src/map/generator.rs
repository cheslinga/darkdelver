use crate::prelude::*;
use bracket_lib::prelude::*;
use std::cmp::{max, min};

#[derive(Clone, Copy, PartialEq)]
pub enum GenerationMode {
    RandomRooms,
    Unimplemented
}

pub struct MapGenerator {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub depth: i32,
    pub max_rooms: usize,
    pub start_pos: Point,
    pub objects: Vec<Object>
}
impl MapGenerator {
    //Public-facing map generation function
    pub fn generate(mode: GenerationMode, w: i32, h: i32, depth: i32, rng: &mut RandomNumberGenerator) -> MapGenerator {
        match mode {
            GenerationMode::RandomRooms => return MapGenerator::random_rooms_build(w, h, depth, rng),
            _ => panic!("Unrecognized map generation mode!")
        }
    }

    //Internal base constructor
    fn init(w: i32, h: i32, depth: i32, max_rooms: usize ) -> MapGenerator {
        MapGenerator { map: Map::new(w, h), rooms: Vec::new(), depth, max_rooms, start_pos: Point::zero(), objects: Vec::new() }
    }

    //Fills the map with the specified tile class
    fn fill(&mut self, tile: TileClass) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile)
    }

    //Helpers that build horizontal and vertical tunnels
    fn make_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_index(x, y) {
                self.map.tiles[idx] = TileClass::Floor;
            }
        }
    }
    fn make_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_index(x, y) {
                self.map.tiles[idx] = TileClass::Floor;
            }
        }
    }

    /* RANDOMLY PLACED ROOMS */
    //Builds a map using randomly placed rooms
    fn random_rooms_build(w: i32, h: i32, depth: i32, rng: &mut RandomNumberGenerator) -> MapGenerator {
        let mut gen = MapGenerator::init(w, h, depth, 20);
        let mut block_list: Vec<Point> = Vec::new();

        //Run all the map-making procedures
        gen.fill(TileClass::Wall);
        gen.make_randomly_placed_rooms(rng);
        gen.add_corridors_sorted(rng);

        //Set the start position
        gen.start_pos = gen.rooms[0].center();

        //Place stairs as the last room's center
        let last_center = gen.map.point2d_to_index(gen.rooms[gen.rooms.len()-1].center());
        gen.map.tiles[last_center] = TileClass::DownStair;

        //Add vectors to track object positions for proximity calculation and room usage for even spawning
        let mut proximity_list: Vec<Point> = Vec::new();
        let mut room_nums: Vec<usize> = Vec::new();
        //Start spawning enemies
        let enemy_spawns = get_enemy_spawn_table(depth, gen.rooms.len() as i32 - 1, rng);
        for (i, _) in gen.rooms.iter().enumerate().skip(1) {
            let mut obj = enemy_spawns[i - 1].clone();
            if let Some(pos) = find_valid_spawn(&gen.rooms, &mut room_nums, &obj, &block_list, Some(&proximity_list), rng) {
                add_positional_info(&mut obj, pos, depth);
                block_list.push(pos);
                proximity_list.push(pos);
                gen.objects.push(obj)
            }
        }

        //Reassign fresh vectors for proximity and room usage tracking
        proximity_list = Vec::new();
        room_nums = Vec::new();
        //Start spawning items
        let item_spawns = get_item_spawns(depth, rng);
        for item in item_spawns.iter() {
            let mut obj = item.clone();
            if let Some(pos) = find_valid_spawn(&gen.rooms, &mut room_nums, &obj, &block_list, Some(&proximity_list), rng) {
                add_positional_info(&mut obj, pos, depth);
                block_list.push(pos);
                proximity_list.push(pos);
                gen.objects.push(obj);
            }
        }

        return gen;
    }
    //Creates some rects of flooring to create rooms
    fn make_randomly_placed_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < self.max_rooms {
            let room = Rect::with_size(
                rng.range(1, self.map.width - 10),
                rng.range(1, self.map.height - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < self.map.width && p.y > 0 && p.y < self.map.height {
                        let idx = self.map.index(p.x, p.y);
                        self.map.tiles[idx] = TileClass::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }
    //Sorts the rooms in the generator struct and tunnels between them
    fn add_corridors_sorted(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_center = rooms[i - 1].center();
            let this_center = room.center();

            match rng.range(0, 2) {
                0 => {
                    self.make_v_tunnel(prev_center.y, this_center.y, prev_center.x);
                    self.make_h_tunnel(prev_center.x, this_center.x, this_center.y);
                }
                _ => {
                    self.make_h_tunnel(prev_center.x, this_center.x, prev_center.y);
                    self.make_v_tunnel(prev_center.y, this_center.y, this_center.x);
                }
            }
        }
    }
}

fn find_valid_spawn(rooms: &Vec<Rect>, room_nums: &mut Vec<usize>, obj: &Object, block_list: &Vec<Point>, proximity_list: Option<&Vec<Point>>, rng: &mut RandomNumberGenerator) -> Option<Point> {
    let mut iter_cnt: u16 = 1024;

    let proximity_graph = {
        let mut graph: Vec<Point> = Vec::new();

        if let Some(list) = proximity_list {
            for p in list.iter() {
                graph.push(*p);
                for i in 1..=5 {
                    graph.append(&mut p.get_distant_neighbors(i));
                }
            }
        }

        graph
    };

    loop {
        if room_nums.len() < 1 { *room_nums = (1 as usize..rooms.len()).map(|x| x).collect::<Vec<usize>>(); }

        let rand_num = rng.range(0, room_nums.len());
        let room_num = room_nums.remove(rand_num);
        let block_graph = {
            let mut graph: Vec<Point> = Vec::new();
            graph.append(&mut block_list.to_vec());
            graph.append(&mut proximity_graph.to_vec());
            graph
        };
        let pos = try_find_spawnable_position(&rooms[room_num],
                                              &block_graph, obj.block_tile, rng);

        if pos.is_some() { return pos }

        iter_cnt -= 1;
        if iter_cnt < 1 { break }
    }
    None
}