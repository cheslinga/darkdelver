use crate::prelude::*;

//Runs all draw batching functions;
pub fn batch_all(map: &Map, camera: &Camera, objects: &Vec<Object>, logs: &LogBuffer, floor: i32, mouse_pos: Point) {
    batch_map_draws(map, camera);
    batch_entity_draws(objects, map, camera, floor);
    batch_mouse_area(mouse_pos);
    batch_ui_draws(&objects[0], logs);
}

//Adds all map tiles to the rendering batch.
fn batch_map_draws(map: &Map, camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(OBJ_LAYER);

    for y in camera.min_y ..= camera.max_y {
        for x in camera.min_x ..= camera.max_x {
            let pos = Point::new(x, y);
            let offset = Point::new(camera.min_x, camera.min_y);

            if map.in_bounds(x,y) {
                let idx = map.index(x,y);
                
                let (glyph, colors) = match (map.visible[idx], map.revealed[idx]) {
                    (true, _)       =>  {get_tile_render(&map.tiles[idx])},
                    (false, true)   =>  {(get_tile_render(&map.tiles[idx]).0, ColorPair::new(GREY10,BLACK))},
                    (false, false)  =>  {(0,ColorPair::new(BLACK,BLACK))},
                };

                batch.set(pos - offset, colors, glyph);
            }
            else {
                batch.set(pos - offset, ColorPair::new(BLACK, BLACK), 0);
            }
        }
    }
    batch.submit(0).expect("Failed to batch map draw");
}

//Adds all visible entity renderables to the rendering batch.
fn batch_entity_draws(objects: &Vec<Object>, map: &Map, camera: &Camera, floor: i32) {
    let mut batch = DrawBatch::new();
    batch.target(OBJ_LAYER);
    let offset = Point::new(camera.min_x, camera.min_y);

    //Grab all objects that are drawable and have a position (force the player in at the end)
    let mut render_list: Vec<(&Object, bool)> = Vec::new();
    for object in objects.iter() {
        if object.pos.is_some() && object.render.is_some() {
            let pos = object.pos.as_ref().unwrap();
            let idx = map.index(pos.x, pos.y);
            if pos.x > camera.min_x && pos.x < camera.max_x && pos.y > camera.min_y && pos.y < camera.max_y && object.floor == floor {
                if map.visible[idx] {
                    render_list.push((object, true))
                } else if map.revealed[idx] && object.player_mem.seen {
                    render_list.push((object, false))
                }
            }
        }
    }

    render_list.sort_by_key(|o| o.0.render.as_ref().unwrap().order);
    for obj in render_list.iter() {
        let pos: Point;
        let mut render: Render;

        if obj.1 {
            pos = obj.0.pos.unwrap();
            render = obj.0.render.unwrap();
        }
        else {
            pos = obj.0.player_mem.last_pos.unwrap();
            render = obj.0.render.unwrap();
            render.color = ColorPair::new(GREY30, BLACK);
        }

        batch.set(pos - offset, render.color, render.glyph);
    }

    batch.submit(5000).expect("Failed to batch entity draw");
}

fn batch_mouse_area(pos: Point) {
    let mut batch = DrawBatch::new();
    batch.target(OBJ_LAYER);
    batch.set_bg(pos, ORANGE);
    batch.submit(5050).expect("Failed to batch mouse draw");
}

fn batch_ui_draws(player: &Object, logs: &LogBuffer) {
    let mut uibatch = DrawBatch::new();
    let mut textbatch = DrawBatch::new();
    uibatch.target(OBJ_LAYER);
    textbatch.target(TXT_LAYER);

    //Draw the stats box
    uibatch.draw_double_box(Rect::with_size(CONSOLE_W - UI_CUTOFF.x, 0, UI_CUTOFF.x - 1, CONSOLE_H - 1), ColorPair::new(GREY75, BLACK));
    textbatch.print(Point::new(CONSOLE_W * 2 - UI_CUTOFF.x * 2 + 4, 0), "Stats");

    if let Some(ActorTag::Player) = player.tag {
        let health = player.health.as_ref().unwrap().current;
        let max = player.health.as_ref().unwrap().max;

        let dmg_string = {
            let mut s = format!("{}d{}", player.damage.as_ref().unwrap().dice, player.damage.as_ref().unwrap().val);
            if !player.damage.as_ref().unwrap().modifiers.is_empty() {
                let mut total = 0;
                for i in player.damage.as_ref().unwrap().modifiers.iter() {
                    total += i;
                }
                s.push_str(format!(" + {}", total).as_str());
            }
            s
        };

        let percent = ((health as f32 / max as f32) * 100.0).round() as i32;
        let colors = if percent <= 25 { ColorPair::new(BLACK, RED) }
                        else if percent <= 50 { ColorPair::new(RED, BLACK) }
                        else if percent <= 75 { ColorPair::new(YELLOW, BLACK) }
                        else { ColorPair::new(WHITE, BLACK) };

        textbatch.print(Point::new(CONSOLE_W * 2 - UI_CUTOFF.x * 2 + 4, 2), "Health:");
        textbatch.print_color(Point::new(CONSOLE_W * 2 - UI_CUTOFF.x * 2 + 4, 3), format!("{}/{}", health, max), colors);

        textbatch.print(Point::new(CONSOLE_W * 2 - UI_CUTOFF.x * 2 + 4, 5), "Damage:");
        textbatch.print(Point::new(CONSOLE_W * 2 - UI_CUTOFF.x * 2 + 4, 6), dmg_string);
    }

    //Draw the log box
    uibatch.draw_double_box(Rect::with_size(0, CONSOLE_H - UI_CUTOFF.y, CONSOLE_W - UI_CUTOFF.x - 1, UI_CUTOFF.y - 1), ColorPair::new(GREY75, BLACK));
    textbatch.print(Point::new(12, CONSOLE_H - UI_CUTOFF.y), "Logs");

    let mut tb = TextBlock::new(2, CONSOLE_H - UI_CUTOFF.y + 1, CONSOLE_W * 2 - UI_CUTOFF.x * 2 - 4, UI_CUTOFF.y - 2);
    tb.print(&logs.format());
    tb.render_to_draw_batch(&mut *textbatch);

    uibatch.submit(10000).expect("Failed to batch UI draw");
    textbatch.submit(15000).expect("Failed to batch UI draw");
}

//Returns glyph and color pair info for a tile.
//TODO: Make tuple globals for map theming.
fn get_tile_render(tile: &TileClass) -> (FontCharType, ColorPair) {
    match tile {
        TileClass::Floor        =>  (46, ColorPair::new(WHITE,BLACK)),
        TileClass::Wall         =>  (176, ColorPair::new(CHOCOLATE4, BLACK)),
        TileClass::DownStair    =>  (62, ColorPair::new(GREY70,GREY99)),
        _                       =>  (0, ColorPair::new(WHITE,BLACK))
    }
}


/*
//Uncomment this for heatmap visual testing because it's a pain in the ass to debug through the console

fn batch_heatmaps_test(objects: &Vec<Object>, map: &Map, camera: &Camera, floor: i32) {
    let mut batch = DrawBatch::new();
    batch.target(1);
    let offset = Point::new(camera.min_x, camera.min_y);

    for obj in objects.iter() {
        if let Some(ai) = &obj.ai {
            for node in ai.tgt_heatmap.nodes.iter() {
                batch.set(*node - offset, ColorPair::new(YELLOW, BLACK), to_cp437('!'));
            }
            for node in ai.tgt_heatmap.old_nodes.iter() {
                batch.set(*node - offset, ColorPair::new(ORANGE, BLACK), to_cp437('?'));
            }
        }
    }

    batch.submit(10000);
}
 */
