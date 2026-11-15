use crate::coords::Coord;
use crate::directions::flat::FlatDirection;
use crate::directions::pointy::PointyDirection;

#[derive(Copy, Clone, Debug)]
pub enum Direction{
    Flat(FlatDirection),
    Pointy(PointyDirection),
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Flat(d) => {Direction::Flat(d.opposite())}
            Direction::Pointy(d) => {Direction::Pointy(d.opposite())}
        }

    }

    pub fn rotate(&self, steps: i32)->Direction{
        match self {
            Direction::Flat(d) => {Direction::Flat(d.rotate(steps))}
            Direction::Pointy(d) => {Direction::Pointy(d.rotate(steps))}
        }
    }


    pub fn offset(&self) -> (i32, i32){
        match self{
            Direction::Flat(d) => d.offset(),
            Direction::Pointy(d) => d.offset(),
        }
    }

    pub fn coord(&self) -> Coord{
        let (q, r) = self.offset();
        Coord::axial(q, r)
    }
}