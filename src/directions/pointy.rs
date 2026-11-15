use crate::coords::Coord;

#[derive(Debug, Copy, Clone)]
pub enum PointyDirection{
    NorthEast,
    NorthWest,
    East,
    West,
    SouthEast,
    SouthWest,
    None
}

impl PointyDirection {
    pub fn new(coord: Coord) -> Self{
        let (q, r) = coord.axial_index();
        match (q,r) {
            (1, -1) => {PointyDirection::NorthEast}
            (0, -1) => {PointyDirection::NorthEast}
            (1, 0) => {PointyDirection::East}
            (-1, 0) => {PointyDirection::West}
            (0, 1) => {PointyDirection::SouthEast}
            (-1, 1) => {PointyDirection::SouthWest}
            (_, _) => {PointyDirection::None}
        }

    }

    pub fn opposite(&self) -> Self{
        match self {
            PointyDirection::NorthEast => {PointyDirection::SouthWest}
            PointyDirection::NorthWest => {PointyDirection::SouthEast}
            PointyDirection::East => {PointyDirection::West}
            PointyDirection::West => {PointyDirection::East}
            PointyDirection::SouthEast => {PointyDirection::NorthWest}
            PointyDirection::SouthWest => {PointyDirection::NorthEast}
            PointyDirection::None => {PointyDirection::None}
        }
    }

    pub fn rotate(&self, steps:i32) -> Self {
        let mut rotated = *self;
        if steps <0{
            for _ in 0..steps{
                rotated = rotated.back();
            }
        }
        else{
            for _ in 0..steps{
                rotated = rotated.next();
            }
        }
        rotated
    }

    fn next(& self) -> Self {
        match self {
            PointyDirection::NorthEast => {PointyDirection::East}
            PointyDirection::NorthWest => {PointyDirection::NorthWest}
            PointyDirection::East => {PointyDirection::SouthEast}
            PointyDirection::West => {PointyDirection::NorthWest}
            PointyDirection::SouthEast => {PointyDirection::SouthWest}
            PointyDirection::SouthWest => {PointyDirection::West}
            PointyDirection::None => {PointyDirection::None}
        }
    }

    fn back(& self) -> Self {
        match self {
            PointyDirection::NorthEast => {PointyDirection::NorthWest}
            PointyDirection::NorthWest => {PointyDirection::West}
            PointyDirection::East => {PointyDirection::NorthEast}
            PointyDirection::West => {PointyDirection::SouthWest}
            PointyDirection::SouthEast => {PointyDirection::East}
            PointyDirection::SouthWest => {PointyDirection::SouthEast}
            PointyDirection::None => {PointyDirection::None}
        }
    }

    pub fn offset(&self) -> (i32, i32){
        match self {
            PointyDirection::NorthEast => {(1,-1)}
            PointyDirection::NorthWest => {(0, -1)}
            PointyDirection::East => {(1,0)}
            PointyDirection::West => {(-1,0)}
            PointyDirection::SouthEast => {(0,1)}
            PointyDirection::SouthWest => {(-1,1)}
            PointyDirection::None => {(0,0)}
        }
    }


}