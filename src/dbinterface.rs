use crate::prelude::*;
use rusqlite::*;
use rusqlite::types::ValueRef;

const DB_FILEPATH: &str = "res/dd_raw.sqlite";

pub trait SqlStringImport {
    fn match_db_string(db_string: String) -> Option<Self> where Self: Sized;
}

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
    let mut main_q = conn.prepare(
        "SELECT * FROM Items T1
            INNER JOIN Renders T2 on T1.render_id = T2.id
            INNER JOIN Interactions T3 on T1.interactions_id = T3.id"
    ).unwrap();

    for item in main_q.query_map(params![], |row| {
        Ok(ExportedObject {
            id: row.get("id")?,
            name: row.get("name")?,
            render_glyph: row.get("glyph")?,
            render_fg: (row.get("fg_r")?, row.get("fg_g")?, row.get("fg_b")?),
            render_bg: (row.get("bg_r")?, row.get("bg_g")?, row.get("bg_b")?),
            item_stats: {
                let mut stats = ItemStats::blank_with_drop();

                if row.get_raw_checked("activation_id")? != ValueRef::Null {
                    import_item_functions(&mut stats, ItemUsage::Activate, vec![ItemEffect::nil()]) }

                if row.get_raw_checked("drink_id")? != ValueRef::Null {
                    import_item_functions(&mut stats, ItemUsage::Drink, vec![ItemEffect::nil()]) }

                if row.get_raw_checked("equip_id")? != ValueRef::Null {
                    let mut equip_q = conn.prepare(
                        format!("SELECT * FROM EquipEffects WHERE id = {}", row.get("equip_id").unwrap_or(0)).as_str()
                    ).unwrap();
                    let effect_ids = get_effect_ids(&mut equip_q, 8).unwrap_or(vec![]);
                    let id_string = {
                        let mut s = String::new();
                        for id in effect_ids.into_iter() {
                            s.push_str(format!("{},", id).as_str());
                        }
                        s.pop();
                        s
                    };

                    let mut effects_q = conn.prepare(
                        format!("SELECT * FROM EffectTable WHERE id IN ({})", id_string).as_str()
                    ).unwrap();
                    let item_effects = get_effects(&mut effects_q).unwrap_or(vec![ItemEffect::nil()]);

                    import_item_functions(&mut stats, ItemUsage::Equip, item_effects)
                }

                Some(stats)
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

fn import_item_functions(stats: &mut ItemStats, usage_val: ItemUsage, mut effect_vals: Vec<ItemEffect>) {
    stats.usages.push(usage_val);
    stats.effects.append(&mut effect_vals);
}

fn get_effect_ids(query: &mut Statement, max_effects: i32) -> Option<Vec<i32>> {
    let mut id_vec = Vec::new();
    for item in query.query_map(params![], |row| {
        Ok({
            id_vec.push(row.get("effect1_id").unwrap_or(0));

            for i in 2..=max_effects {
                if row.get_raw_checked(format!("effect{}_id", i).as_str())? != ValueRef::Null {
                    id_vec.push(row.get(format!("effect{}_id", i).as_str()).unwrap_or(0)); }
            }
            })
    }).ok()? {
        return Some(id_vec)
    }
    None
}

fn get_effects(query: &mut Statement) -> Option<Vec<ItemEffect>> {
    let mut effects = Vec::new();
    for item in query.query_map(params![], |row| {
        Ok({
            let etype = EffectType::match_db_string(
                row.get("effect").unwrap_or("NIL".to_string())
            ).unwrap_or(EffectType::NIL);

            effects.push(
                ItemEffect {
                    //Assign the effect type
                    etype,
                    //Assign the parameter values
                    params: {
                        let mut param_vec = Vec::new();
                        for i in 1..=16 {
                            if row.get_raw_checked(format!("param{}", i).as_str())? != ValueRef::Null {
                                param_vec.push(row.get(format!("param{}", i).as_str()).unwrap_or(0));
                            }
                        }
                        if !param_vec.is_empty() { Some(param_vec) } else { None }
                    },
                    //Determine whether the effect takes place on equip
                    on_equip: match etype {
                        EffectType::WeaponDamage => true,
                        _ => false
                    }
                }
            );
        })
    }).ok()? {
        return Some(effects)
    }
    None
}