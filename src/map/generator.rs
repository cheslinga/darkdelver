use bracket_lib::prelude::*;
use super::mapdef::*;
use std::cmp::{min,max};

const MAX_RANDROOMS: usize = 20;

pub struct MapGenerator {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub start_pos: Point
}
impl MapGenerator {
    //Fills the map with the specified tile class
    fn fill(&mut self, tile: TileClass) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile)
    }
    //Helpers that build horizontal and vertical tunnels
    fn make_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1,x2)..=max(x1,x2) {
            if let Some(idx) = self.map.try_index(x, y) {
                self.map.tiles[idx] = TileClass::Floor;
            }
        }
    }
    fn make_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1,y2)..=max(y1,y2) {
            if let Some(idx) = self.map.try_index(x, y) {
                self.map.tiles[idx] = TileClass::Floor;
            }
        }
    }

/* RANDOMLY PLACED ROOMS */
    //Builds a map using randomly placed rooms
    pub fn random_rooms_build(w: i32, h: i32, rng: &mut RandomNumberGenerator) -> MapGenerator {
        let mut gen = MapGenerator {
            map: Map::new(w,h),
            rooms: Vec::new(),
            start_pos: Point::zero()
        };

        gen.fill(TileClass::Wall);
        gen.make_randomly_placed_rooms(rng);
        gen.add_corridors_sorted(rng);
        gen.start_pos = gen.rooms[0].center();
        return gen
    }
    //Creates some rects of flooring to create rooms
    fn make_randomly_placed_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < MAX_RANDROOMS {
            let room = Rect::with_size(
                rng.range(1, self.map.width - 10),
                rng.range(1, self.map.height - 10),
                rng.range(2,10),
                rng.range(2,10)
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
                        let idx = self.map.index(p.x,p.y);
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
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_center = rooms[i-1].center();
            let this_center = room.center();

            match rng.range(0,2) {
                0 => {
                    self.make_v_tunnel(prev_center.y, this_center.y, prev_center.x);
                    self.make_h_tunnel(prev_center.x, this_center.x, this_center.y);
                },
                _ => {
                    self.make_h_tunnel(prev_center.x, this_center.x, prev_center.y);
                    self.make_v_tunnel(prev_center.y, this_center.y, this_center.x);
                }
            }
        }
    }
}
