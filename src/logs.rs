use crate::prelude::*;

pub type LogBuffer = Vec<LogMessage>;

pub trait LogBufferTrait {
    fn format(&self) -> TextBuilder;
    fn update_logs(&mut self, message: LogMessage);
}

impl LogBufferTrait for LogBuffer {
    fn format(&self) -> TextBuilder {
        let mut builder = TextBuilder::empty();

        for (n, message) in self.iter().enumerate() {
            for (i, part) in message.parts.iter().enumerate() {
                let bg = message.colors[i].bg;
                if n % 2 == 0 { builder.bg(bg + RGBA::from_f32(0.05,0.05,0.05,bg.a)); }
                else { builder.bg(bg); }
                builder.fg(message.colors[i].fg);
                builder.line_wrap(part);
            }
            builder.ln();
        }

        return builder;
    }
    fn update_logs(&mut self, message: LogMessage) {
        if self.len() > 13 { self.remove(0); }
        self.push(message);
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