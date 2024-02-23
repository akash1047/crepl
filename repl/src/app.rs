use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};

#[derive(Debug)]
pub struct App {
    stdout: Stdout,
    height: u16,
    width: u16,

    redraw: bool,
}

const LOGO: &'static str = r#"
   _____   _____                   _      
  / ____| |  __ \                 | |     
 | |      | |__) |   ___   _ __   | |     
 | |      |  _  /   / _ \ | '_ \  | |     
 | |____  | | \ \  |  __/ | |_) | | |____ 
  \_____| |_|  \_\  \___| | .__/  |______|
                          | |             
                          |_|             "#;

impl App {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            redraw: true,
            height: 0,
            width: 0,
        }
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        (self.width, self.height) = crossterm::terminal::size()?;
        self.redraw = true;

        crossterm::execute!(stdout(), EnterAlternateScreen, Hide)?;
        crossterm::terminal::enable_raw_mode()?;

        loop {
            if self.redraw {
                queue!(
                    self.stdout,
                    crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
                )?;

                // queue!(self.stdout, MoveTo(0, 0), Print("C R e p L"))?;
                // queue!(self.stdout, MoveTo(0, 0), Print(""))?;

                Text::new(LOGO).draw(
                    &mut self.stdout,
                    Surface {
                        x: 0,
                        y: 0,
                        width: self.width,
                        height: self.height,
                    },
                )?;

                let inner = Border::ROUNDED.draw(
                    &mut self.stdout,
                    Surface {
                        x: 0,
                        y: 10,
                        width: self.width,
                        height: self.height,
                    },
                )?;

                Text::new("press ctrl + d to exit").draw(&mut self.stdout, inner)?;

                self.stdout.flush()?;
                self.redraw = false;
            }

            // event handles
            if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(250)) {
                match crossterm::event::read()? {
                    // closing
                    Event::Key(KeyEvent {
                        code:
                            KeyCode::Char('d')
                            | KeyCode::Char('D')
                            | KeyCode::Char('c')
                            | KeyCode::Char('C'),
                        kind: KeyEventKind::Press,
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => {
                        break;
                    }

                    Event::Resize(x, y) => {
                        self.width = x;
                        self.height = y;
                        self.redraw = true;
                    }

                    _ => {}
                }
            }
        }

        crossterm::execute!(stdout(), Show, LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Surface {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Border(
    char, // top left corner
    char, // top
    char, // top right corner
    char, // right
    char, // bot right corner
    char, // bot
    char, // bot left corner
    char, // left
);

impl Border {
    const ASCII: Border = Border('+', '-', '+', '|', '+', '-', '+', '|');
    const ROUNDED: Border = Border('╭', '─', '╮', '│', '╯', '─', '╰', '│');
    const YAHO: Border = Border('a', '&', 'a', 'p', 'x', 'c', 'y', 't');

    pub fn draw<W: QueueableCommand>(
        &self,
        writer: &mut W,
        surf: Surface,
    ) -> std::io::Result<Surface> {
        // top left corner
        writer.queue(MoveTo(surf.x, surf.y))?;
        writer.queue(Print(self.0))?;

        // top line
        for i in surf.x + 1..surf.width - 1 {
            writer.queue(MoveTo(i, surf.y))?;
            writer.queue(Print(self.1))?;
        }

        // top right corner
        writer.queue(MoveTo(surf.width - 1, surf.y))?;
        writer.queue(Print(self.2))?;

        // right side
        for j in surf.y + 1..surf.height - 1 {
            writer.queue(MoveTo(surf.width - 1, j))?;
            writer.queue(Print(self.3))?;
        }

        // bot right corner
        writer.queue(MoveTo(surf.width - 1, surf.height - 1))?;
        writer.queue(Print(self.4))?;

        // bot line
        for i in surf.x + 1..surf.width - 1 {
            writer.queue(MoveTo(i, surf.height - 1))?;
            writer.queue(Print(self.5))?;
        }

        // bot left corner
        writer.queue(MoveTo(surf.x, surf.height - 1))?;
        writer.queue(Print(self.6))?;

        // left side
        for j in surf.y + 1..surf.height - 1 {
            writer.queue(MoveTo(surf.x, j))?;
            writer.queue(Print(self.7))?;
        }

        Ok(Surface {
            x: surf.x + 1,
            y: surf.y + 1,
            width: surf.width - 1,
            height: surf.height - 1,
        })
    }
}

pub struct Text {
    lines: Vec<String>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            lines: text.lines().map(|s| s.to_string()).collect(),
        }
    }

    pub fn draw<W: QueueableCommand>(&self, writer: &mut W, surf: Surface) -> std::io::Result<()> {
        for (i, l) in self
            .lines
            .iter()
            .map(|s| String::from_iter(s.chars().take(surf.width as usize)))
            .take(surf.height as usize)
            .enumerate()
        {
            writer.queue(MoveTo(surf.x, surf.y + i as u16))?;
            writer.queue(Print(l))?;
        }

        Ok(())
    }
}
