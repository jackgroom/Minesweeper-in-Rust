mod vec2;
mod cell;
mod board;

use std::io::stdout;
use vec2::*;
use cell::*;
use board::*;

use crossterm::event::{poll, read, Event, KeyEventKind, KeyCode};
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const WIDTH: usize = 10;
const HEIGHT: usize = 5;

fn setup_input_event(board: &mut Board) -> crossterm::Result<()> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char('w') => { board.update_player_pos(Vec2::new(0, -1)) }
                            KeyCode::Char('a') => { board.update_player_pos(Vec2::new(-1, 0)) }
                            KeyCode::Char('s') => { board.update_player_pos(Vec2::new(0, 1)) }
                            KeyCode::Char('d') => { board.update_player_pos(Vec2::new(1, 0)) }
                            KeyCode::Char('f') => { println!("f") }
                            KeyCode::Char(' ') => { println!("space") }
                            _ => {}
                        }
                        board.display();
                    }
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() {
    // let _board: Board<Cell> = Board::<Cell>::new(WIDTH, HEIGHT); -- used to have generic board type
    let mut _board: Board = Board::new(WIDTH, HEIGHT);
    _board.display();

    if let Err(e) = setup_input_event(&mut _board) {
        println!("Error: {:?}\r", e);
    }
}