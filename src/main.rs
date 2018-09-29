extern crate piston_window;

mod board;
mod draw;
mod game;

use piston_window::types::Color;
use piston_window::{clear, PistonWindow, UpdateEvent, WindowSettings};

use self::draw::to_coord_u32;
use self::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (50, 50);

    let mut game = Game::new(width, height);
    game.init(&vec![(2, 3), (3, 4), (4, 4), (4, 3), (4, 2)]);

    let mut window: PistonWindow =
        WindowSettings::new("Rrrrrife!", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
