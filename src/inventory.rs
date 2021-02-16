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
        for obj in objects.iter() {
            if let Some(inv) = &obj.in_inventory {
                if inv.owner_id == 0 {
                    let info = ItemInfo {
                       name: obj.name.as_ref().unwrap().to_owned(),
                       render: *&obj.render.unwrap()
                    };
                    self.items.push(info);
                }
            }
        }
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

    let menubox = Rect::with_size(2, 2, CONSOLE_W - UI_CUTOFF.x - 12, min(menu.items.len() as i32 + 1, 18));
    batch.draw_double_box(menubox, ColorPair::new(GREY75, BLACK));
    batch.print(Point::new(4, 2), "Inventory");
    batch.print_color(Point::new(4, 2 + menubox.height()), "ESC to close", ColorPair::new(GOLD4,BLACK));

    let mut y = menubox.y1 + 1;
    for item in menu.items.iter() {
        if y < menubox.y2 {
            batch.set(Point::new(menubox.x1 + 1, y), *&item.render.color, item.render.glyph);
            batch.print(Point::new(menubox.x1 + 3, y), &item.name);
            y += 1;
        }
    }

    batch.submit(11000).expect("Failed to batch inventory menu draw");
}