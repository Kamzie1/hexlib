use crate::coords::Coord;

#[derive(Copy, Clone, Debug)]
pub enum FlatDirection{
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    None
}

impl FlatDirection {
    pub fn new(coord: Coord) -> Self{
        let (q, r) = coord.axial_index();
        match (q,r) {
            (1, -1) => {FlatDirection::NorthEast }
            (0, -1) => {FlatDirection::North}
            (1, 0) => {FlatDirection::SouthEast}
            (-1, 0) => {FlatDirection::NorthWest}
            (0, 1) => {FlatDirection::South}
            (-1, 1) => {FlatDirection::SouthWest}
            (_, _) => {FlatDirection::None}
        }

    }
    pub fn opposite(&self) -> FlatDirection {
        match self{
            FlatDirection::North => {FlatDirection::South}
            FlatDirection::NorthWest => {FlatDirection::SouthEast }
            FlatDirection::NorthEast => {FlatDirection::SouthWest }
            FlatDirection::South => {FlatDirection::North}
            FlatDirection::SouthWest => {FlatDirection::NorthEast }
            FlatDirection::SouthEast => {FlatDirection::NorthWest }
            FlatDirection::None => {FlatDirection::None}
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
            FlatDirection::NorthEast => {FlatDirection::SouthEast}
            FlatDirection::NorthWest => {FlatDirection::North}
            FlatDirection::North => {FlatDirection::NorthEast}
            FlatDirection::South => {FlatDirection::SouthWest}
            FlatDirection::SouthEast => {FlatDirection::South}
            FlatDirection::SouthWest => {FlatDirection::NorthWest}
            FlatDirection::None => {FlatDirection::None}
        }
    }

    fn back(& self) -> Self {
        match self {
            FlatDirection::NorthEast => {FlatDirection::North}
            FlatDirection::NorthWest => { FlatDirection::SouthWest}
            FlatDirection::North => {FlatDirection::NorthWest}
            FlatDirection::South => {FlatDirection::SouthEast}
            FlatDirection::SouthEast => {FlatDirection::NorthEast}
            FlatDirection::SouthWest => {FlatDirection::South}
            FlatDirection::None => {FlatDirection::None}
        }
    }

    pub fn offset(&self) -> (i32, i32){
        match self {
            FlatDirection::NorthEast => {(1,-1)}
            FlatDirection::NorthWest => {(-1, 0)}
            FlatDirection::North => {(0,-1)}
            FlatDirection::South => {(0,1)}
            FlatDirection::SouthEast => {(1,0)}
            FlatDirection::SouthWest => {(-1,1)}
            FlatDirection::None => {(0,0)}
        }
    }
}