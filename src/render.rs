use crate::prelude::*;

//Runs all draw batching functions;
pub fn batch_all(gs: &mut State) {
    batch_map_draws(&gs.world.active_map, &gs.world.camera);
    batch_entity_draws(&gs.world.objects, &gs.world.camera);
}

//Adds all map tiles to the rendering batch.
fn batch_map_draws(map: &Map, camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(0);

    for y in camera.min_y ..= camera.max_y {
        for x in camera.min_x ..= camera.max_x {
            let pos = Point::new(x, y);
            let offset = Point::new(camera.min_x, camera.min_y);

            if map.in_bounds(x,y) {
                let idx = map.index(x,y);
                
                let (glyph, colors) = match (map.visible[idx], map.revealed[idx]) {
                    (true, _) => {get_tile_render(&map.tiles[idx])},
                    (false, true) => {(get_tile_render(&map.tiles[idx]).0, ColorPair::new(GREY10,BLACK))},
                    (false, false) => {(0,ColorPair::new(BLACK,BLACK))},
                };

                batch.set(pos - offset, colors, glyph);
            }
        }
    }
    batch.submit(0).expect("Failed to batch map draw");
}

//Adds all visible entity renderables to the rendering batch.
fn batch_entity_draws(objects: &Vec<Object>, camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(0);
    let offset = Point::new(camera.min_x, camera.min_y);

    //Grab all objects that are drawable and have a position (force the player in at the end)
    let mut render_list: Vec<&Object> = Vec::new();
    for object in objects.iter() {
        if let Object{pos: Some(_), render: Some(_), ..} = object {
            render_list.push(object)
        }
    }

    for obj in render_list.iter() {
        let pos = obj.pos.unwrap();
        let render = obj.render.as_ref().unwrap();
        batch.set(pos - offset, render.color, render.glyph);
    }

    batch.submit(5000).expect("Failed to batch entity draw");
}

//Returns glyph and color pair info for a tile.
//TODO: Make tuple globals for map theming.
fn get_tile_render(tile: &TileClass) -> (FontCharType, ColorPair) {
    match tile {
        TileClass::Floor => (46, ColorPair::new(WHITE,BLACK)),
        TileClass::Wall => (35, ColorPair::new(BLACK,DARK_SLATE))
    }
}
