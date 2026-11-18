use crate::coords::*;
#[derive(Debug, Hash, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub struct CartesianCoord{
    pub x:i32,
    pub y:i32,
}

impl CartesianCoord {
    pub fn new(x:i32,y:i32)->CartesianCoord{
        CartesianCoord{x,y}
    }

    pub fn axial(&self) -> AxialCoord {
        let q = self.x;
        let r = self.y - (q - q.abs() % 2) / 2;
        AxialCoord::new(q, r)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axial() {
        let cart = CartesianCoord::new(1, 2);
        let axial = cart.axial();
        let expected = AxialCoord::new(1, 2);
        assert_eq!(expected, axial);
    }
}
