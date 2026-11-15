use crate::coords::Coord;

pub struct Map<T, const ROWS: usize, const COLS: usize> {
    tiles: [[T; COLS]; ROWS],
}

impl<T: Copy, const ROWS: usize, const COLS: usize> Map<T, ROWS, COLS> {
    pub fn new(default: T) -> Self {
        Self {
            tiles: [[default; COLS]; ROWS],
        }
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        let(x, y) = coord.array2d_index()?;
        self.tiles.get(x)?.get(y)
    }
}