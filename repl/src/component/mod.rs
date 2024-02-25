use std::io::Write;

use crossterm::{
    cursor::MoveTo,
    style::{Color, Colors, Print, ResetColor, SetColors},
    QueueableCommand,
};

use crate::app::{Drawable, Surface};

#[derive(Debug, Default)]
pub struct Text {
    text: String,

    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl<T: AsRef<str>> From<T> for Text {
    fn from(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
            ..Self::default()
        }
    }
}

impl Drawable for Text {
    type Out = Surface;

    fn draw(&self, write: &mut dyn Write, surf: crate::app::Surface) -> std::io::Result<Self::Out> {
        let mut buffer = vec![vec![' '; surf.width as usize]; surf.height as usize];

        for (j, l) in self.text.lines().enumerate() {
            for (i, c) in l.chars().enumerate() {
                if i < surf.width as usize {
                    buffer[j][i] = c;
                }
            }
        }

        for (i, s) in buffer.iter().map(|s| String::from_iter(s)).enumerate() {
            write.queue(SetColors(Colors {
                foreground: self.fg,
                background: self.bg,
            }))?;

            write.queue(MoveTo(surf.x, surf.y + i as u16))?;
            write.queue(Print(s))?;
        }

        write.queue(ResetColor)?;

        Ok(surf)
    }
}

impl Text {
    pub fn size(&self) -> (u16, u16) {
        let mut width = 0;
        let mut height = 0;

        self.text.lines().for_each(|s| {
            height += 1;
            width = s.len().max(width);
        });

        (width as u16, height)
    }
}
