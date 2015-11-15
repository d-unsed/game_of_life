use std::thread;

mod cell;
mod coordinate;
mod neighbours;
mod world;

use coordinate::Coordinate;
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
        print!("{}[", '\x1B');
        print!("2J");
        println!("{}", world.to_string(Coordinate(0, 0), Coordinate(30, 30)));

        world = world.next_generation();

        thread::sleep_ms(100);
    }
}
