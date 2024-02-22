use std::{io::stdout, time::Duration};

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

fn main() -> std::io::Result<()> {
    let logo = format!(
        "Hello {}! This is crepl.\npress 'Escape' or 'q' to exit",
        whoami::realname()
    );

    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            f.render_widget(
                Paragraph::new(logo.as_str()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                f.size(),
            );
        })?;

        if crossterm::event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    _ => {}
                }
            }
        }
    }

    crossterm::execute!(stdout(), LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
