use crate::prelude::*;
use rusqlite::*;

const DB_FILEPATH: &str = "res/dd_raw.sqlite";

struct ExportedObject {
    id: u32,
    name: String,
    render_glyph: u8,
    render_fg: (u8,u8,u8),
    render_bg: (u8,u8,u8)
}

pub fn open_connection() -> Connection {
    return Connection::open(DB_FILEPATH).unwrap()
}

pub fn import_items_to_objects(conn: &Connection) -> Option<Vec<Object>> {
    let mut objs: Vec<Object> = Vec::new();
    let mut command = conn.prepare(
        "SELECT
            *
        FROM
            Items T1
        INNER JOIN
            Renders T2
        on
            T1.render_id = T2.id
        INNER JOIN
            Interactions T3
        on
            T1.interactions_id = T3.id"
    ).unwrap();

    for item in command.query_map(params![], |row| {
        Ok(ExportedObject {
            id: row.get(0)?,
            name: row.get(1)?,
            render_glyph: row.get(7)?,
            render_fg: (row.get(8)?, row.get(9)?, row.get(10)?),
            render_bg: (row.get(11)?, row.get(12)?, row.get(13)?)
        })
    }).ok()?
    {
        if let Ok(exp) = item {
            let fg = RGBA::from_u8(exp.render_fg.0, exp.render_fg.1, exp.render_fg.2, 255);
            let bg = RGBA::from_u8(exp.render_bg.0, exp.render_bg.1, exp.render_bg.2, 255);

            let obj = Object {
                name: Some(exp.name),
                db_imported: Some(exp.id),
                render: Some(Render {
                    glyph: exp.render_glyph as u16,
                    color: ColorPair::new(fg, bg),
                    order: 3
                }),
                ..Default::default()
            };
            objs.push(obj);
        }
    }
    return Some(objs)
}