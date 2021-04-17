use crate::prelude::*;
use rusqlite::*;
use rusqlite::types::ValueRef;

const DB_FILEPATH: &str = "res/dd_raw.sqlite";

struct ExportedObject {
    id: u32,
    name: String,
    render_glyph: u8,
    render_fg: (u8,u8,u8),
    render_bg: (u8,u8,u8),
    item_stats: Option<ItemStats>,
    equip_slot: Option<EquipSlot>
}

pub fn open_connection() -> Connection {
    return Connection::open(DB_FILEPATH).expect("Connection to SQLite DB could not be opened. Please check the 'res/' folder for the file 'dd_raw.sqlite'.")
}

pub fn import_items_to_objects(conn: &Connection) -> Option<Vec<Object>> {
    let mut objs: Vec<Object> = Vec::new();
    let mut command = conn.prepare(
        "SELECT * FROM Items T1
            INNER JOIN Renders T2 on T1.render_id = T2.id
            INNER JOIN Interactions T3 on T1.interactions_id = T3.id"
    ).unwrap();

    for item in command.query_map(params![], |row| {
        Ok(ExportedObject {
            id: row.get("id")?,
            name: row.get("name")?,
            render_glyph: row.get("glyph")?,
            render_fg: (row.get("fg_r")?, row.get("fg_g")?, row.get("fg_b")?),
            render_bg: (row.get("bg_r")?, row.get("bg_g")?, row.get("bg_b")?),
            item_stats: {
                let mut stats = ItemStats::blank_with_drop();

                if row.get_raw_checked("activation_id")? != ValueRef::Null { import_item_functions(&mut stats, ItemUsage::Activate, ItemEffect::nil()) }
                if row.get_raw_checked("drink_id")? != ValueRef::Null { import_item_functions(&mut stats, ItemUsage::Drink, ItemEffect::nil()) }
                if row.get_raw_checked("equip_id")? != ValueRef::Null { import_item_functions(&mut stats, ItemUsage::Equip, ItemEffect::nil()) }

                if stats.usages.is_empty() { None }
                else { Some(stats) }
            },
            equip_slot: EquipSlot::match_db_string(row.get("item_slot").unwrap_or(format!("NIL")))
        })
    }).ok()? {
        if let Ok(exp) = item {
            let fg = RGBA::from_u8(exp.render_fg.0, exp.render_fg.1, exp.render_fg.2, 255);
            let bg = RGBA::from_u8(exp.render_bg.0, exp.render_bg.1, exp.render_bg.2, 255);

            let obj = Object {
                name: Some(exp.name),
                render: Some(Render {
                    glyph: exp.render_glyph as u16,
                    color: ColorPair::new(fg, bg),
                    order: 3
                }),
                item_stats: exp.item_stats,
                equip_slot: exp.equip_slot,
                ..Default::default()
            };
            objs.push(obj);
        }
    }
    return Some(objs)
}

fn import_item_functions(stats: &mut ItemStats, usage_val: ItemUsage, effect_val: ItemEffect) {
    stats.usages.push(usage_val);
    stats.effects.push(effect_val);
}