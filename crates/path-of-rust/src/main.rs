use std::io;
use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
    style::{Color, Style},
    widgets::Paragraph,
    DefaultTerminal, Frame, 
};

// player movement
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// holds player information including starting location,
struct Player {
    x: u16,
    y: u16,
}

// this will store game information between when frames are rendered
struct Game {
    player: Player,
    exit: bool,
}

impl Game {
    // player starts at 20, 10
    fn new() -> Self {
        Self {
            player: Player { x: 20, y: 10 },
            exit: false,
        }
    }
    
    // apply one move step for the player
    fn move_player(&mut self, direction: Direction) {
        let (dx, dy): (i32, i32) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        self.player.x = (i32::from(self.player.x) + dx).max(0) as u16;
        self.player.y = (i32::from(self.player.y) + dy).max(0) as u16;
    }

    // handle pending input from movement keys, 1/Esc to quit
    fn handle_events(&mut self) -> io::Result<()> {
        if !event::poll(Duration::from_millis(50))? {
            return Ok(());
        }
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('w') | KeyCode::Up => self.move_player(Direction::Up),
                    KeyCode::Char('s') | KeyCode::Down => self.move_player(Direction::Down),
                    KeyCode::Char('a') | KeyCode::Left => self.move_player(Direction::Left),
                    KeyCode::Char('d') | KeyCode::Right => self.move_player(Direction::Right),
                    KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

// this draws a frame of the rendering
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

// main game loop: will redraw and wait up to 50ms for a key press or Esc to quit
fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut game = Game::new();
    while !game.exit {
        terminal.draw(|frame| draw(frame, &game))?;
        game.handle_events()?;
    }
    Ok(())
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
