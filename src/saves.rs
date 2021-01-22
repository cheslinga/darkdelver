use crate::prelude::*;
use std::fs::*;
use std::io::*;

pub fn export_world(world: &World) {
    let jsonstring = serde_json::to_string(world).expect("Could not serialize world data!");

    let mut file = File::create("saves/current.json").expect("Could not create save file!");
    file.write_all(jsonstring.as_bytes())
        .expect("Could not write to file!");
}

pub fn load_world(gs: &mut State) {
    let mut file = File::open("saves/current.json").expect("Could not open file!");
    let mut filestring = String::new();
    file.read_to_string(&mut filestring).unwrap();

    let deserialized_world: World = serde_json::from_str(&filestring).unwrap();
    gs.world = deserialized_world;
}
