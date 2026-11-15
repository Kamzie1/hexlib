use crate::coords::Coord;
use crate::directions::direction::Direction;

pub struct Neighbors {
    pub center: Coord,
    pub direction: Direction,
    index: usize,
}

impl Neighbors {
    pub fn new(center: Coord, direction: Direction) -> Self {
        Neighbors{center, direction, index: 0}
    }
}

impl Iterator for Neighbors {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 6 {
            let (dq, dr) = self.direction.offset();
            self.index += 1;
            Some(Coord::axial(self.center.axial_index().0 + dq, self.center.axial_index().1 + dr))
        } else {
            None
        }
    }
}
