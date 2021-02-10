use crate::prelude::*;

const MAX_WIDTH: usize = (CONSOLE_W - UI_CUTOFF.x - 2) as usize;

pub type LogBuffer = Vec<LogMessage>;

pub trait LogBufferTrait {
    fn format(&self) -> TextBuilder;
    fn update_logs(&mut self, message: LogMessage);
}

impl LogBufferTrait for LogBuffer {
    fn format(&self) -> TextBuilder {
        let mut builder = TextBuilder::empty();

        for (n, message) in self.iter().rev().enumerate() {
            for (i, part) in message.parts.iter().enumerate() {
                let bg = message.colors[i].bg;
                builder.bg(bg - RGBA::from_f32(0.01,0.01,0.01,0.0) * n as f32);
                builder.fg(message.colors[i].fg);
                builder.append(part);
            }
            //builder.ln();
        }

        return builder;
    }
    fn update_logs(&mut self, message: LogMessage) {
        let mut group_list: Vec<(String, ColorPair)> = Vec::new();

        for (c, part) in message.parts.iter().enumerate() {
            let split = part.split_whitespace();
            for s in split.into_iter() {
                group_list.push((s.to_string() + " ", message.colors[c]));
            }
        }

        let mut final_messages: Vec<LogMessage> = Vec::new();
        let mut wip_message = LogMessage::new();
        let mut line_len: usize = 0;
        let mut last_color: ColorPair = ColorPair::new(WHITE, GREY10);

        for g in group_list.iter() {
            if line_len + g.0.len() >= MAX_WIDTH {
                while line_len < MAX_WIDTH {
                    wip_message = wip_message.add_part(" ", last_color);
                    line_len += 1;
                }
                final_messages.insert(0, wip_message);

                line_len = g.0.len();
                wip_message = LogMessage::new().add_part(g.0.to_string(), g.1);
            }
            else {
                wip_message = wip_message.add_part(g.0.to_string(), g.1);
                line_len += g.0.len();
            }
            last_color = g.1;
        }
        while line_len < MAX_WIDTH {
            wip_message = wip_message.add_part(" ", last_color);
            line_len += 1;
        }
        final_messages.insert(0, wip_message);

        for message in final_messages.iter() {
            self.push(LogMessage {
                parts: message.parts.to_vec(),
                colors: message.colors.to_vec()
            });
        }
        if self.len() > 31 { self.remove(0); }
    }
}

pub struct LogMessage {
    pub parts: Vec<String>,
    pub colors: Vec<ColorPair>
}
impl LogMessage {
    pub fn new() -> LogMessage {
        LogMessage {
            parts: Vec::new(),
            colors: Vec::new()
        }
    }
    pub fn add_part<T: ToString>(mut self, part: T, color: ColorPair) -> Self {
        self.parts.push(part.to_string());
        self.colors.push(color);

        return self
    }
}