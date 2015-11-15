use coordinate::Coordinate;
use world::World;

pub struct Display<'a> {
    world: &'a World,
    top_left: Coordinate,
    bottom_right: Coordinate,
}

impl<'a> Display<'a> {
    pub fn new(world: &'a World, top_left: Coordinate, bottom_right: Coordinate) -> Self {
        Display {
            world: world,
            top_left: top_left,
            bottom_right: bottom_right
        }
    }

    pub fn render(&self) {
        print!("{}[", '\x1B');
        print!("2J");
        println!("{}", self.to_string());
    }

    fn to_string(&self) -> String {
        let Coordinate(left_x, top_y) = self.top_left;
        let Coordinate(right_x, bottom_y) = self.bottom_right;

        (left_x..right_x).map(|i| {
            (top_y..bottom_y).map(|j| {
                self.world.get(Coordinate(i, j)).to_char()
            }).collect::<String>()
        }).collect::<Vec<String>>()
        .connect("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::Display;
    use coordinate::Coordinate;
    use world::World;

    #[test]
    fn it_converts_world_to_string() {
        let world = World::new(vec![Coordinate(0, 1), Coordinate(1, 1), Coordinate(2, 1)]);
        let display = Display::new(&world, Coordinate(0, 0), Coordinate(3, 3));

        assert_eq!(display.to_string(), "+X+\n+X+\n+X+".to_string());
    }
}
