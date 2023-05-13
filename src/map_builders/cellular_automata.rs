/////////// ------------------------------------------------------///////////
///                                                                       ///
///                        Cellular Automata!                             ///
///                                                                       ///
/////////// ------------------------------------------------------///////////

use super::MapBuilder;
use super::Map;
use super::map::TileType;


pub struct CellularAutomataBuilder {
    map : Map,
    iterations: i32,
}

impl<'a> MapBuilder<'a> for CellularAutomataBuilder {
    fn build(&mut self) {
        self.scramble();
    }

    fn iterate(&mut self) {
        if self.iterations < 16 {
            self.iterate();
            self.iterations += 1;
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn update_map_data(&self, map_data: &mut Vec<(&'a str, String)>) {
        map_data.clear();
        map_data.push(("Name", "Cellular Automata".to_string()));
        map_data.push(("Max Iterations", format!("{}", 16)));
        map_data.push(("Iteration", format!("{}", self.iterations)));
    }

    fn notes(&self) -> &str {
        ""
    }
}

impl CellularAutomataBuilder {
    pub fn new() -> CellularAutomataBuilder {
        CellularAutomataBuilder {
            map : Map::new(),
            iterations: 0,
        }
    }

    pub fn scramble(&mut self) {
        self.map.tiles.clear();
        self.iterations = 0;

        for _r in 0..self.map.height {
            for _c in 0..self.map.width {
                self.map.tiles.push(
                    match fastrand::i32(0..2) {
                        0 => TileType::Floor,
                        _ => TileType::Wall
                    });
            }
        }
    }

    pub fn iterate(&mut self) {
        let mut new_tiles = Vec::<TileType>::new();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let num_neighbors = self.count_neighbors(x, y);
    
                if num_neighbors > 4 {
                    new_tiles.push(TileType::Floor);
                }
                else if (0..=4).contains(&num_neighbors) {
                    new_tiles.push(TileType::Wall);
                }
            }
        }
        self.map.tiles = new_tiles;
    }
    
    fn count_neighbors(&self, x: usize, y: usize) -> i32 {
        let mut num_neighbors = 0;
    
        // X . .
        // . . .
        // . . .
        if x > 0 && y > 0 { num_neighbors += self.has_neighbor( x - 1, y - 1); }
    
        // . . .
        // X . .
        // . . .
        if x > 0 { num_neighbors += self.has_neighbor(x - 1, y); }
    
        // . . .
        // . . .
        // X . .
        if x > 0 && y < self.map.height - 1 { num_neighbors += self.has_neighbor( x - 1, y + 1); }
    
        // . . .
        // . . X
        // . . .
        if x < self.map.width - 1 { num_neighbors += self.has_neighbor(x + 1, y); }
    
        // . . .
        // . . .
        // . X .
        if y < self.map.height - 1 { num_neighbors += self.has_neighbor(x, y + 1); }
    
        // . X .
        // . . .
        // . . .
        if y > 0 { num_neighbors += self.has_neighbor(x, y - 1); }
    
        // . . X
        // . . .
        // . . .
        if x < self.map.width - 1 && y > 0 { num_neighbors += self.has_neighbor(x + 1, y - 1); }
    
        // . . .
        // . . .
        // . . X
        if x < self.map.width - 1 && y < self.map.height - 1 { num_neighbors += self.has_neighbor(x + 1, y + 1); }
    
        num_neighbors
    }
    
    fn has_neighbor(&self, x: usize, y: usize) -> i32 {
        match self.map.get(x, y) {
            Some(tile) => match tile {
                TileType::Wall => 1,
                TileType::Floor => 0
            },
            None => {
                print!("Couldn't find {x}, {y}");
                0
            }
        }
    }
}

impl Default for CellularAutomataBuilder {
    fn default() -> Self {
        Self::new()
    }
}