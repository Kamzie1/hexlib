use crate::coords::*;
use crate::directions::direction::Direction;
use crate::directions::pointy::PointyDirection;
use crate::neighbours::Neighbors;

#[derive(Debug, Hash, Clone, Copy)]
pub enum Coord {
    Cartesian(CartesianCoord),
    Axial(AxialCoord),
}

impl Coord {
    pub fn cartesian(x: i32, y: i32) -> Coord {
        Coord::Cartesian(CartesianCoord::new(x, y))
    }

    pub fn axial(q: i32, r: i32) -> Coord {
        Coord::Axial(AxialCoord::new(q, r))
    }

    pub fn to_cartesian_coord(&self) -> CartesianCoord{
        match self {
            Coord::Cartesian(c) => { *c }
            Coord::Axial(c) => { c.cartesian() }
        }
    }
    pub fn to_cartesian(&self) -> Coord {
        match self {
            Coord::Cartesian(_) => { *self }
            Coord::Axial(c) => { Coord::Cartesian(c.cartesian()) }
        }
    }

    pub fn to_axial_coord(&self) -> AxialCoord {
        match self {
            Coord::Cartesian(c) => { c.axial()}
            Coord::Axial(c) => { *c }
        }
    }
    pub fn to_axial(&self) -> Coord {
        match self {
            Coord::Cartesian(c) => { Coord::Axial(c.axial())}
            Coord::Axial(_) => { *self }
        }
    }

    pub fn neighbours(&self, direction:Direction) -> Neighbors {
        Neighbors::new(self.to_axial(), direction)
    }

    pub fn cartesian_index(&self) -> (i32, i32) {
        match self {
            Coord::Cartesian(c) => {
                (c.x, c.y)
            }
            Coord::Axial(c) => {
                let coord = c.cartesian();
                (coord.x, coord.y)
            }
        }
    }
    pub fn array2d_index(&self) -> Option<(usize, usize)> {
        match self {
            Coord::Cartesian(c) => {
                let row = usize::try_from(c.x).ok()?;
                let col = usize::try_from(c.y).ok()?;
                Some((row, col))
            }
            Coord::Axial(c) => {
                let coord = c.cartesian();
                let row = usize::try_from(coord.x).ok()?;
                let col = usize::try_from(coord.y).ok()?;
                Some((row, col))
            }
        }
    }

    pub fn axial_index(&self) -> (i32, i32) {
        match self {
            Coord::Cartesian(c) => {
                let coord = c.axial();
                (coord.q, coord.r)
            }
            Coord::Axial(c) => {
                (c.q, c.r)
            }
        }
    }

    pub fn is_neighbour(&self, coord:&Coord) -> bool {
        self.neighbours(Direction::Pointy(PointyDirection::NorthEast)).any(|c| &c == coord)
    }

    pub fn in_range(&self, vec:&Vec<Coord>) -> bool {
        vec.iter().any(|c| c == self)
    }

    pub fn manhattan_distance(&self, coord:&Coord) -> i32 {
        let(x1, y1) = self.cartesian_index();
        let(x2, y2) = coord.cartesian_index();
        (x2 - x1).abs() + (y2 - y1).abs()
    }

    pub fn distance(&self, coord:&Coord) -> i32 {
        let(x1, y1) = self.cartesian_index();
        let(x2, y2) = coord.cartesian_index();
        (x2 - x1).pow(2) + (y2 - y1).pow(2)
    }

    pub fn move_in_direction(&self, direction:Direction, steps:i32) -> Coord {
        let dir= if steps < 0{
            direction.opposite()
        } else{
            direction
        };
        Coord::Axial(self.to_axial_coord().plus(dir.offset().0 * steps, dir.offset().1 * steps))
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        let cartesian = self.to_cartesian_coord();
        let other_cartesian = other.to_cartesian_coord();
        cartesian == other_cartesian
    }
}

impl Eq for Coord {}