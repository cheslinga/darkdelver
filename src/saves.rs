use crate::prelude::*;
use std::fs::*;
use std::io::*;

pub fn export_world(world: &World) {
    let raw_data = serde_cbor::ser::to_vec(world).expect("Could not serialize world data!");
    let mut file = File::create("saves/current.save").expect("Could not create save file!");
    file.write_all(&*raw_data)
        .expect("Could not write to file!");
}

pub fn load_world(gs: &mut State) {
    let mut file = File::open("saves/current.save").expect("Could not open file!");
    let deserialized_world: World = serde_cbor::de::from_reader(&file).unwrap();
    gs.world = deserialized_world;
}