use crate::prelude::*;
use std::cmp::min;

pub struct InventoryMenu {
    pub submenu: Option<InventorySubMenu>,
    pub items: Vec<ItemInfo>,
    pub selection: usize
}
pub struct InventorySubMenu {
    pub text: String,
}
pub struct ItemInfo {
    pub obj_id: usize,
    pub name: String,
    pub render: Render
}

impl InventoryMenu {
    pub fn new(objects: &Vec<Object>) -> InventoryMenu {
        let mut menu = InventoryMenu {..Default::default()};
        menu.populate_items(objects);
        return menu
    }
    pub fn populate_items(&mut self, objects: &Vec<Object>) {
        for (i, obj) in objects.iter().enumerate() {
            if let Some(inv) = &obj.in_inventory {
                if inv.owner_id == 0 {
                    let info = ItemInfo {
                        obj_id: i,
                        name: obj.name.as_ref().unwrap().to_owned(),
                        render: *&obj.render.unwrap()
                    };
                    self.items.push(info);
                }
            }
        }
    }
    pub fn process_selection(&mut self, objects: &mut Vec<Object>) {
        let item_ptr = &mut self.items[self.selection];
        let obj_ptr = &mut objects[item_ptr.obj_id];

        //TEST STUFF:
        console::log(format!("Hey, you selected this item! {}", obj_ptr.name.as_ref().unwrap_or(&"No Name???".to_string())));
    }
}
impl Default for InventoryMenu {
    fn default() -> InventoryMenu {
        InventoryMenu {
            submenu: None,
            items: Vec::new(),
            selection: 0
        }
    }
}

pub fn batch_inventory_menu(menu: &InventoryMenu) {
    let mut batch = DrawBatch::new();
    batch.target(0);

    let menubox = Rect::with_size(2, 2, CONSOLE_W - UI_CUTOFF.x - 4, min(menu.items.len() as i32 + 1, 18));
    batch.draw_double_box(menubox, ColorPair::new(GREY75, BLACK));
    batch.print(Point::new(4, 2), "Inventory");
    batch.print_color(Point::new(4, 2 + menubox.height()), "ESC to close", ColorPair::new(GOLD4,BLACK));

    let mut y = menubox.y1 + 1;
    let mut ofs: u16 = 0;
    for (i, item) in menu.items.iter().enumerate() {
        let line_color = match i == menu.selection {
            false => {ColorPair::new(WHITE,BLACK)},
            true => {ColorPair::new(BLACK,YELLOW)}
        };
        if y < menubox.y2 {
            batch.set(Point::new(menubox.x1 + 5, y), *&item.render.color, item.render.glyph);

            //Sets the letter to display next to the item
            batch.set(Point::new(menubox.x1 + 1, y), ColorPair::new(WHITE,BLACK), 40);
            batch.set(Point::new(menubox.x1 + 2, y), ColorPair::new(GOLD2, BLACK), 97 + ofs);
            batch.set(Point::new(menubox.x1 + 3, y), ColorPair::new(WHITE,BLACK), 41);

            batch.print_color(Point::new(menubox.x1 + 7, y), &item.name, line_color);
            y += 1;
            ofs += 1;
        }
    }

    batch.submit(11000).expect("Failed to batch inventory menu draw");
}