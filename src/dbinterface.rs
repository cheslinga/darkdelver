use crate::prelude::*;
use rusqlite::*;
use rusqlite::types::ValueRef;

const DB_FILEPATH: &str = "res/dd_raw.sqlite";

pub trait SqlStringImport {
    fn match_db_string(db_string: String) -> Option<Self> where Self: Sized;
}

pub struct SpawnTableEntry { pub enemy_id: u32, pub weight: u8 }

struct ExportedObject {
    id: u32,
    name: String,
    render_glyph: u16,
    render_order: u8,
    render_fg: (u8,u8,u8),
    render_bg: (u8,u8,u8),
    health: Option<i32>,
    damage: Option<(i32, i32)>,
    item_stats: Option<ItemStats>,
    equip_slot: Option<EquipSlot>,
    view_range: Option<i32>,
    initiative: Option<u8>
}

pub fn open_connection() -> Connection {
    return Connection::open(DB_FILEPATH).expect("Connection to SQLite DB could not be opened. Please check the 'res/' folder for the file 'dd_raw.sqlite'.")
}

pub fn get_spawn_table_info(conn: &Connection, spawn_table: u32) -> Option<Vec<SpawnTableEntry>> {
    let mut entries = Vec::new();

    let mut main_q = build_initial_query(conn, String::from("SpawnTables"), Some(format!("st_id = {}", spawn_table)));

    for entry in main_q.query_map(params![], |row| {
        Ok(
            (row.get("enemy")?, row.get("spawn_weight")?)
        )
    }).ok()? {
        if let Ok((enemy_id, weight)) = entry {
            entries.push(SpawnTableEntry { enemy_id, weight });
        }
    }
    //entries.sort_by(|a,b| a.weight.cmp(&b.weight));
    return Some(entries)
}

pub fn import_enemies_to_objects(conn: &Connection, table: String, where_args: Option<String>) -> Option<Vec<Object>> {
    let mut objs: Vec<Object> = Vec::new();

    let mut main_q = build_initial_query(conn, table, where_args);

    for enemy in main_q.query_map(params![], |row| {
        Ok(ExportedObject{
            id: row.get("id")?,
            name: row.get("name")?,
            render_glyph: row.get("glyph")?,
            render_order: row.get("render_order")?,
            render_fg: (row.get("fg_r")?, row.get("fg_g")?, row.get("fg_b")?),
            render_bg: (row.get("bg_r")?, row.get("bg_g")?, row.get("bg_b")?),
            health: Some(row.get("health")?),
            damage: Some((row.get("damage_dice")?, row.get("damage_sides")?)),
            item_stats: None,
            equip_slot: None,
            view_range: Some(row.get("view_range")?),
            initiative: Some(row.get("initiative")?)
        })
    }).ok()? {
        if let Ok(exp) = enemy {
            let fg = RGBA::from_u8(exp.render_fg.0, exp.render_fg.1, exp.render_fg.2, 255);
            let bg = RGBA::from_u8(exp.render_bg.0, exp.render_bg.1, exp.render_bg.2, 255);

            let obj = Object {
                name: Some(exp.name),
                render: Some(Render {
                    glyph: exp.render_glyph as u16,
                    color: ColorPair::new(fg, bg),
                    order: exp.render_order
                }),
                health: Some(Health::new(exp.health.unwrap())),
                damage: Some(Damage::new(exp.damage.unwrap())),

                tag: Some(ActorTag::Enemy),
                viewshed: Some(Viewshed {
                    range: exp.view_range.unwrap(),
                    visible: Vec::new(),
                    refresh: true,
                }),
                block_tile: true,
                initiative: exp.initiative,
                ai: Some(AIClass::new()),
                ..Default::default()
            };
            objs.push(obj);
        }
    }

    return Some(objs)
}

pub fn import_items_to_objects(conn: &Connection, table: String, where_args: Option<String>) -> Option<Vec<Object>> {
    let mut objs: Vec<Object> = Vec::new();

    let mut main_q = build_initial_query(conn, table, where_args);

    for item in main_q.query_map(params![], |row| {
        Ok(ExportedObject {
            id: row.get("id")?,
            name: row.get("name")?,
            render_glyph: row.get("glyph")?,
            render_order: row.get("render_order")?,
            render_fg: (row.get("fg_r")?, row.get("fg_g")?, row.get("fg_b")?),
            render_bg: (row.get("bg_r")?, row.get("bg_g")?, row.get("bg_b")?),
            health: None,
            damage: None,
            item_stats: {
                let mut stats = ItemStats::blank_with_drop();

                if row.get_raw_checked("activation_id")? != ValueRef::Null {
                    import_item_functions(&mut stats, ItemUsage::Activate, import_effects(conn, row, ItemUsage::Activate, 8))
                }
                if row.get_raw_checked("drink_id")? != ValueRef::Null {
                    import_item_functions(&mut stats, ItemUsage::Drink, import_effects(conn, row, ItemUsage::Drink, 8))
                }
                if row.get_raw_checked("equip_id")? != ValueRef::Null {
                    import_item_functions(&mut stats, ItemUsage::Equip, import_effects(conn, row, ItemUsage::Equip, 8))
                }

                Some(stats)
            },
            equip_slot: EquipSlot::match_db_string(row.get("item_slot").unwrap_or(format!("NIL"))),
            view_range: None,
            initiative: None
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
                    order: exp.render_order
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

fn build_initial_query(conn: &Connection, table: String, where_args: Option<String>) -> Statement {
    return {
        let query_str = {
            let mut wrk = format!("SELECT * FROM {}", table);
            if let Some(args) = where_args {
                wrk = format!("{} WHERE {}", wrk.clone(), args)
            }
            wrk
        };
        conn.prepare(query_str.as_str()).expect(format!("Failed to execute query '{}' against SQLite database.", query_str).as_str())
    }
}

fn import_effects(conn: &Connection, row: &Row, usage: ItemUsage, max_effects: i32) -> Vec<ItemEffect> {
    let (table, row_key) = match usage {
        ItemUsage::Equip => ("EquipEffects", "equip_id"),
        ItemUsage::Drink => ("DrinkEffects", "drink_id"),
        ItemUsage::Activate => ("ActivationEffects", "activation_id"),
        _ => ("", "")
    };
    if table == "" { return vec![ItemEffect::nil()] }

    let mut main_q = conn.prepare(
        format!("SELECT * FROM {} WHERE id = {}", table, row.get(row_key).unwrap_or(0)).as_str()
    ).unwrap();

    let effect_ids = get_effect_ids(&mut main_q, max_effects).unwrap_or(vec![]);
    let id_string = make_id_string(&effect_ids);
    let mut effects_q = prep_effect_query(conn, id_string);

    return get_effects(&mut effects_q).unwrap_or(vec![ItemEffect::nil()])
}

fn make_id_string(ids: &Vec<i32>) -> String {
    //Builds up a list of ID integers in string format
    let mut s = String::new();
    for id in ids.into_iter() { s.push_str(format!("{},", id).as_str()); }
    s.pop();
    s
}

fn prep_effect_query(conn: &Connection, id_list: String) -> Statement {
    conn.prepare(format!("SELECT * FROM EffectTable WHERE id IN ({})", id_list).as_str()).unwrap()
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