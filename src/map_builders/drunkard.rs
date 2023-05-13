/////////// ------------------------------------------------------///////////
///                                                                       ///
///                             Drunkard!                                 ///
///                                                                       ///
/////////// ------------------------------------------------------///////////

use rand::Rng;

use super::MapBuilder;
use super::Map;
use super::map::TileType;


pub struct DrunkardBuilder {
    map : Map,
    iterations: i32,
    max_steps: i32,
}

impl<'a> MapBuilder<'a> for DrunkardBuilder {
    fn build(&mut self) {
        self.clear();
        self.seed();
    }

    fn iterate(&mut self) {
        if self.iterations < 24 {
            self.iterate();
            self.iterations += 1;
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn update_map_data(&self, map_data: &mut Vec<(&'a str, String)>) {
        map_data.clear();
        let num_walls = self.num_walls();
        map_data.push(("Name", "Drunkard's Walk".to_string()));
        map_data.push(("Max Iterations", format!("{}", 24)));
        map_data.push(("Steps Per Iteration", format!("{}", self.max_steps)));
        map_data.push(("Iteration", format!("{}", self.iterations)));
        map_data.push(("Total Number of Walls", format!("{}", num_walls)));
        map_data.push(("Total Empty Space", format!("{}", 1600 - num_walls)));
        map_data.push(("% Occupied", format!("{}", num_walls as f32 / 1600_f32)));
    }

    fn notes(&self) -> &str {
        ""
    }
}

impl DrunkardBuilder {
    pub fn new() -> DrunkardBuilder {
        DrunkardBuilder {
            map : Map::new(),
            iterations: 0,
            max_steps: 100,
        }
    }

    pub fn clear(&mut self) {
        self.iterations = 0;
        self.map = Map::new();
    }

    pub fn seed(&mut self) {
        let x = self.map.width / 2;
        let y = self.map.height / 2;

        // create seed in center:
        // . X .
        // X X X
        // . X .
        self.map.set(TileType::Floor, x, y - 1);
        self.map.set(TileType::Floor, x - 1, y);
        self.map.set(TileType::Floor, x, y);
        self.map.set(TileType::Floor, x + 1, y);
        self.map.set(TileType::Floor, x, y + 1);
    }

    pub fn iterate(&mut self) {
        let mut x: usize;
        let mut y: usize;
        let mut rng = rand::thread_rng();
        const START_RADIUS: usize = 10;

        loop {
            x = rng.gen_range(START_RADIUS..(self.map.width - START_RADIUS));
            y = rng.gen_range(START_RADIUS..(self.map.height - START_RADIUS));
            if self.map.get(x, y) == Some(&TileType::Floor){
                break;
            }
        }

        for _ in 0..self.max_steps {
            // check bounds
            if x == 0 || y == 0 || x == self.map.width - 1 || y == self.map.height - 1 {
                break;
            }

            // choose a direction and take a step
            match rng.gen_range(0..4) {
                // Up
                0 => { y -= 1; },
                // Down
                1 => { y += 1; },
                // Left
                2 => { x -= 1; },
                // Right
                _ => { x += 1; }
            }
    
            // check for floor
            if let Some(tile) = self.map.get(x, y) {
                if tile == &TileType::Wall {
                    self.map.set(TileType::Floor, x, y);
                }
            } 
        }
    }

    fn num_walls(&self) -> i32 {   
        let mut num_walls = 0;
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if let Some(tile) = self.map.get(x, y) {
                    if tile.eq(&TileType::Wall) {
                        num_walls += 1;
                    }
                } 
            }
        }
        num_walls
    }
}

impl Default for DrunkardBuilder {
    fn default() -> Self {
        Self::new()
    }
}