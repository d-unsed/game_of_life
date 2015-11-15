#[macro_use] extern crate itertools;

use std::thread;

mod cell;
mod coordinate;
mod display;
mod neighbours;
mod world;

use coordinate::Coordinate;
use display::Display;
use world::World;

fn main() {
    let coords = vec![
        Coordinate(10, 11),
        Coordinate(11, 11),
        Coordinate(12, 11),
        Coordinate(12, 12),
        Coordinate(13, 13),
        Coordinate(14, 14)
    ];

    let mut world = World::new(coords);

    loop {
        Display::new(&world, Coordinate(0, 0), Coordinate(30, 30)).render();

        world = world.next_generation();

        thread::sleep_ms(100);
    }
}
