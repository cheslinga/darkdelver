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
                if n % 2 == 0 { builder.bg(bg + RGBA::from_f32(0.06,0.06,0.06,0.0)); }
                else { builder.bg(bg); }
                builder.fg(message.colors[i].fg);
                builder.append(part);
            }
            builder.ln();
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
        for g in group_list.iter() {
            if line_len + g.0.len() >= MAX_WIDTH {
                line_len = g.0.len();

                let last_char: usize = wip_message.parts.len() - 1;
                wip_message.parts[last_char].pop();

                final_messages.insert(0, wip_message);
                wip_message = LogMessage::new().add_part(g.0.to_string(), g.1);
            }
            else {
                wip_message = wip_message.add_part(g.0.to_string(), g.1);
                line_len += g.0.len();
            }
        }
        let last_char: usize = wip_message.parts.len() - 1;
        wip_message.parts[last_char].pop();
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