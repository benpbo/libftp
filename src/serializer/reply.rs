use std::io::Write;

use crate::reply::{Reply, Text};

use super::Serializer;

pub struct ReplySerializer<W: Write> {
    writer: W,
}

impl<W: Write> ReplySerializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn serialize_reply(&mut self, value: &Reply) -> std::io::Result<()> {
        self.writer.write_all(&value.code)?;
        match &value.text {
            Text::SingleLine { line } => {
                self.writer.write_all(b" ")?;
                self.writer.write_all(line)?;
            }
            Text::MultiLine { lines, last_line } => {
                self.writer.write_all(b"-")?;
                for line in lines {
                    self.writer.write_all(line)?;
                    self.writer.write_all(b"\r\n")?;
                }

                self.writer.write_all(&value.code)?;
                self.writer.write_all(b" ")?;
                self.writer.write_all(last_line)?;
            }
        }

        self.writer.write_all(b"\r\n")
    }
}

impl<W: Write> Serializer<Reply> for ReplySerializer<W> {
    fn serialize(&mut self, value: &Reply) -> std::io::Result<()> {
        self.serialize_reply(value)?;
        self.writer.flush()
    }
}
