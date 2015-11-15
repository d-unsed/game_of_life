#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(pub isize, pub isize);

impl Coordinate {
    pub fn shift_by(&self, shift: Self) -> Self {
        let Coordinate(x, y) = *self;
        let Coordinate(shift_x, shift_y) = shift;

        Coordinate(x + shift_x, y + shift_y)
    }
}

#[cfg(test)]
mod tests {
    use super::Coordinate;

    #[test]
    fn it_shifts_to_correct_position() {
        let start = Coordinate(1, 2);
        let shift = Coordinate(1, -1);

        assert_eq!(start.shift_by(shift), Coordinate(2, 1));
    }
}
