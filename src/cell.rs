use std::ops::Range;

use neighbours::{CellsCount, Neighbours};

struct NeighboursToSurvive(Range<usize>);

pub trait Cell {
    // Include current cell to result to simplify calculations of alive neighbours
    fn suitable_neighbours_count(&self) -> NeighboursToSurvive;
    fn to_char(&self) -> char;

    fn next_generation(&self, neighbours: Neighbours) -> Option<AliveCell> {
        let NeighboursToSurvive(mut r) = self.suitable_neighbours_count();
        let alive_count = neighbours.alive_count();

        r.find(|n| CellsCount(*n) == alive_count).map(|_| AliveCell)
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AliveCell;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DeadCell;

impl Cell for AliveCell {
    fn suitable_neighbours_count(&self) -> NeighboursToSurvive {
        NeighboursToSurvive(3..5)
    }

    fn to_char(&self) -> char {
        'X'
    }
}

impl Cell for DeadCell {
    fn suitable_neighbours_count(&self) -> NeighboursToSurvive {
        NeighboursToSurvive(3..4)
    }

    fn to_char(&self) -> char {
        '+'
    }
}


#[cfg(test)]
mod tests {
    use super::{DeadCell, AliveCell, Cell};
    use neighbours::Neighbours;

    fn alive_cell() -> AliveCell {
        AliveCell
    }

    fn dead_cell() -> DeadCell {
        DeadCell
    }

    fn neighbours<'a>(vec: &'a Vec<AliveCell>) -> Neighbours {
        let vec = vec.iter().map(|cell| cell).collect();

        Neighbours::new(vec)
    }

    #[test]
    fn it_dies_of_underpopulation() {
        let n = vec![AliveCell; 2];

        assert_eq!(alive_cell().next_generation(neighbours(&n)), None);
    }

    #[test]
    fn it_stays_alive_with_2_or_3_neighbours() {
        let n = vec![AliveCell; 3];

        assert_eq!(alive_cell().next_generation(neighbours(&n)), Some(AliveCell));
    }

    #[test]
    fn it_dies_of_overpopulation() {
        let n = vec![AliveCell; 5];

        assert_eq!(alive_cell().next_generation(neighbours(&n)), None);
    }

    #[test]
    fn it_becomes_alive_by_reproduction() {
        let n = vec![AliveCell; 3];

        assert_eq!(dead_cell().next_generation(neighbours(&n)), Some(AliveCell));
    }

    #[test]
    fn it_has_correct_char() {
        assert_eq!(alive_cell().to_char(), 'X');
        assert_eq!(dead_cell().to_char(), '+');
    }
}
