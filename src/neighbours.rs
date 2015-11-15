use cell::AliveCell;

#[derive(Debug, PartialEq, Eq)]
pub struct CellsCount(pub usize);

pub struct Neighbours<'a> {
    cells: Vec<&'a AliveCell>
}

impl<'a> Neighbours<'a> {
    pub fn new(cells: Vec<&'a AliveCell>) -> Self {
        Neighbours {
            cells: cells
        }
    }

    pub fn alive_count(&self) -> CellsCount {
        CellsCount(self.cells.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{CellsCount, Neighbours};
    use cell::AliveCell;

    #[test]
    fn it_counts_alive_neighbours() {
        let cell = AliveCell;

        assert_eq!(Neighbours::new(vec![&cell]).alive_count(), CellsCount(1));
    }
}
