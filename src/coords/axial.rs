use crate::coords::*;
#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AxialCoord {
    pub q:i32,
    pub r:i32,
}
impl AxialCoord {
    pub fn new(q:i32,r:i32)-> AxialCoord {
        AxialCoord {q,r}
    }

    pub fn cartesian(&self) -> CartesianCoord {
        let x = self.q;
        let y =  self.r + (x - x.abs() % 2) / 2;
        CartesianCoord::new(x, y)
    }

    pub fn s(&self) -> i32{
        -self.q - self.r
    }
}
