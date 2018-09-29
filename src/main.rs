#![allow(dead_code)]

extern crate piston_window;
extern crate rand;
extern crate serde_json;

mod board;
mod draw;
mod game;

use piston_window::types::Color;
use piston_window::{clear, PistonWindow, UpdateEvent, WindowSettings};

use rand::{thread_rng, Rng};
use serde_json::Value;
use std::collections::HashSet;
use std::env;
use std::fs;

use self::board::Board;
use self::draw::to_coord_u32;
use self::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn make_coords(board: &Board, n: i32) -> Vec<(i32, i32)> {
    let mut coords: HashSet<(i32, i32)> = HashSet::new();
    let mut rng = thread_rng();

    while coords.len() < (n as usize) {
        coords.insert((
            rng.gen_range(1, board.width - 1),
            rng.gen_range(1, board.height - 1),
        ));
    }

    coords.into_iter().collect()
}

fn load_coords(fname: String) -> Vec<(i32, i32)> {
    let text = fs::read_to_string(fname).unwrap();
    let v: Value = serde_json::from_str(&text).unwrap();
    let pairs: &Vec<Value> = v.as_array().unwrap();
    let mut coords: Vec<(i32, i32)> = Vec::new();

    for pair in pairs {
        let pair = pair.as_array().unwrap();
        assert_eq!(pair.len(), 2);

        coords.push((
            pair[0].as_i64().unwrap() as i32,
            pair[1].as_i64().unwrap() as i32,
        ));
    }

    coords
}

fn main() {
    let (width, height) = (50, 50);
    let mut game = Game::new(width, height);

    let mut args: env::Args = env::args();

    let coords = if args.len() == 2 {
        args.next(); // drop the command path
        load_coords(args.next().unwrap())
    } else {
        make_coords(&game.board, 1000)
    };

    game.init(&coords);

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
