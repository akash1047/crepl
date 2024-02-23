use std::io::{stdout, Stderr, Stdin, Stdout, Write};

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

                queue!(self.stdout, MoveTo(0, 0), Print("C R e p e L"))?;
                queue!(self.stdout, MoveTo(3, 2), Print("press Ctrl+D to exit"))?;

                // Border::ASCII.draw(
                //     &mut self.stdout,
                //     Surface {
                //         x: 2,
                //         y: 2,
                //         width: self.width,
                //         height: self.height,
                //     },
                // )?;

                self.stdout.flush()?;
                self.redraw = false;
            }

            // event handles
            if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(250)) {
                match crossterm::event::read()? {
                    // closing
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('d') | KeyCode::Char('D'),
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
        writer.queue(MoveTo(surf.x, surf.width - 1))?;
        writer.queue(Print(self.2))?;

        Ok(Surface {
            x: surf.x + 1,
            y: surf.y + 1,
            width: surf.width - 1,
            height: surf.height - 1,
        })
    }
}
