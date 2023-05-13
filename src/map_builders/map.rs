use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default, Clone)]
pub struct Map {
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    /// Generates a map filled with walls
    pub fn new() -> Map {
        let width = 39;
        let height = 39;
        Map{
            tiles : vec![TileType::Wall; width * height],
            width,
            height
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&TileType> {
        self.tiles.get(y * self.width + x)
    }

    pub fn set(&mut self, new_tile: TileType, x: usize, y: usize) {
        match self.tiles.get_mut(y * self.height + x) {
            Some(tile) => *tile = new_tile,
            None => print!("Couldn't find tile {x}, {y}")
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = match self.tiles.get(y * self.width + x).expect("Found tiles2") {
                    TileType::Floor => "  ",
                    TileType::Wall => "■ "
                };
                str.push_str(tile);
            }
            str.push('\n');
        }
        write!(f, "{str}")
    }
}