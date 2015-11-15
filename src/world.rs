use itertools::Itertools;

use std::collections::HashMap;

use cell::{Cell, AliveCell, DeadCell};
use coordinate::Coordinate;
use neighbours::Neighbours;

pub struct World {
    alive_cells: HashMap<Coordinate, AliveCell>
}

impl World {
    pub fn new(alive_cells_coordinates: Vec<Coordinate>) -> Self {
        World {
            alive_cells: Self::build_hash_map(alive_cells_coordinates)
        }
    }

    fn build_hash_map(coordinates: Vec<Coordinate>) -> HashMap<Coordinate, AliveCell> {
        coordinates
            .iter()
            .map(|c| (*c, AliveCell))
            .collect()
    }

    pub fn next_generation(&self) -> World {
        let cells = self.alive_cells
            .iter()
            // Get cordinates of all alive cells and their neighbours
            // (potentially alive cells for next generation)
            .flat_map(|(coordinate, _)| {
                iproduct!(-1..2, -1..2).map(|(x, y)| {
                    coordinate.shift_by(Coordinate(x, y))
                }).collect::<Vec<Coordinate>>()
            // Get vectors of alive neighbours for "potentially alive" cells
            }).map(|coordinate| {
                let neighbours =
                    iproduct!(-1..2, -1..2).filter_map(|(x, y)| {
                        self.alive_cells.get(&coordinate.shift_by(Coordinate(x, y)))
                    }).collect::<Vec<&AliveCell>>();

                (coordinate, Neighbours::new(neighbours))
            // Process cells
            }).map(|(coordinate, neighbours)| {
                let current_cell = self.get(coordinate);

                (coordinate, current_cell.next_generation(neighbours))
            })
            // Filter alive cells
            .filter(|&(_, ref cell)| cell.is_some())
            .map(|(coordinate, _)| coordinate)
            .collect();

        World::new(cells)
    }

    pub fn get(&self, coordinate: Coordinate) -> Box<Cell> {
        self.alive_cells
            .get(&coordinate)
            .map_or(Box::new(DeadCell), |cell| Box::new(*cell))
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    use cell::AliveCell;
    use coordinate::Coordinate;

    #[test]
    fn it_creates_new_generation_when_all_cells_die() {
        let coordinates = vec![Coordinate(1, 1)];
        let world = World::new(coordinates);

        let new_world = world.next_generation();

        assert_eq!(new_world.alive_cells.len(), 0);
    }

    #[test]
    fn it_creates_new_generation_when_cells_regenerate() {
        let coordinates = vec![Coordinate(0, 1), Coordinate(1, 1), Coordinate(2, 1)];
        let world = World::new(coordinates);
        let new_world = world.next_generation();
        let new_coordinates: Vec<&Coordinate> = new_world.alive_cells.keys().collect();

        assert_eq!(new_coordinates, vec![&Coordinate(1, 2), &Coordinate(1, 0), &Coordinate(1, 1)]);
    }
}
