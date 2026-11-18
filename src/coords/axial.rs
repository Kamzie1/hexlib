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

    pub fn plus(&self, q:i32, r:i32) -> AxialCoord {
        AxialCoord::new(q + self.q, r + self.r)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cartesian() {
        let axial = AxialCoord::new(0,0);
        let cart  = axial.cartesian();
        let expected = CartesianCoord::new(0,0);
        assert_eq!(expected, cart);
    }

    #[test]
    fn plus(){
        let axial = AxialCoord::new(1,0);
        let got = axial.plus(1,1);
        let expected = AxialCoord::new(2,1);
        assert_eq!(expected, got);
    }

    #[test]
    fn s(){
        let axial = AxialCoord::new(1,0);
        let expected = -1;
        assert_eq!(expected, axial.s());
    }
}
