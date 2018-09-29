use super::board::Board;
use super::draw::draw_block;

use piston_window::types::Color;
use piston_window::{Context, G2d};

const LCELL_COLOR: Color = [0.9, 0.0, 0.0, 1.0];
const MOVING_PERIOD: f64 = 0.1;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            board: Board::new(width, height),
            waiting_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.waiting_time > MOVING_PERIOD {
            self.board = self.board.next().unwrap();
            self.waiting_time = 0.0;
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for c in &self.board.state {
            draw_block(LCELL_COLOR, c.x, c.y, con, g);
        }
    }

    pub fn init(&mut self, initial_state: &Vec<(i32, i32)>) {
        self.board.init(initial_state);
    }
}
