use crate::prelude::*;

//Runs all draw batching functions;
pub fn batch_all(map: &Map, camera: &Camera, objects: &Vec<Object>, floor: i32) {
    batch_map_draws(map, camera);
    batch_entity_draws(objects, map, camera, floor);
    batch_ui_draws();
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
fn batch_entity_draws(objects: &Vec<Object>, map: &Map, camera: &Camera, floor: i32) {
    let mut batch = DrawBatch::new();
    batch.target(0);
    let offset = Point::new(camera.min_x, camera.min_y);

    //Grab all objects that are drawable and have a position (force the player in at the end)
    let mut render_list: Vec<&Object> = Vec::new();
    for object in objects.iter() {
        if let Object{pos: Some(_), render: Some(_), ..} = object {
            let pos = object.pos.as_ref().unwrap();
            let idx = map.index(pos.x, pos.y);
            if map.visible[idx] && pos.x > camera.min_x && pos.x < camera.max_x && pos.y > camera.min_y && pos.y < camera.max_y && object.floor == floor {
                render_list.push(object)
            }
        }
    }

    render_list.sort_by_key(|o| o.render.as_ref().unwrap().order);
    for obj in render_list.iter() {
        let pos = obj.pos.unwrap();
        let render = obj.render.as_ref().unwrap();
        batch.set(pos - offset, render.color, render.glyph);
    }

    batch.submit(5000).expect("Failed to batch entity draw");
}

fn batch_ui_draws() {
    let mut batch = DrawBatch::new();
    batch.target(0);

    batch.draw_box(Rect::with_size(CONSOLE_W - UI_CUTOFF.x, 0, UI_CUTOFF.x - 1, CONSOLE_H - 1), ColorPair::new(GREY75, BLACK));
    batch.draw_box(Rect::with_size(0, CONSOLE_H - UI_CUTOFF.y, CONSOLE_W - UI_CUTOFF.x - 1, UI_CUTOFF.y - 1), ColorPair::new(GREY75, BLACK));

    batch.submit(10000).expect("Failed to batch UI draw");
}

//Returns glyph and color pair info for a tile.
//TODO: Make tuple globals for map theming.
fn get_tile_render(tile: &TileClass) -> (FontCharType, ColorPair) {
    match tile {
        TileClass::Floor => (46, ColorPair::new(WHITE,BLACK)),
        TileClass::Wall => (35, ColorPair::new(BLACK,DARK_SLATE)),
        TileClass::DownStair => (62, ColorPair::new(GREY70,GREY99)),
    }
}


