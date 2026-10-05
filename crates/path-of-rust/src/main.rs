use std::io;
use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
    style::{Color, Style},
    widgets::Paragraph,
    DefaultTerminal, Frame, 
};

// state types
struct Player {
    x: u16,
    y: u16,
}

struct Game {
    player: Player,
}

impl Game {
    fn new() -> Self {
        Self {
            player: Player { x: 20, y: 10 }
        }
    }
}

fn draw(frame: &mut Frame, game: &Game) {
    let area = frame.area();
    frame.render_widget(
        Paragraph::new(format!(
            "Path of Rust - step 1: quit with q (terminal {}x{})",
            area.width, area.height 
        )), 
        area,
    );
    frame.buffer_mut().set_string(
        game.player.x, 
        game.player.y, 
        "@", 
        Style::new().fg(Color::Green),
    );
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let game = Game::new();
    loop {
        terminal.draw(|frame| draw(frame, &game))?;
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn banner_is_set() {
//         assert_eq!(banner(), "Path of Rust");
//     }
// }
