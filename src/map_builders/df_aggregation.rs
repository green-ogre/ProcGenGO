/////////// ------------------------------------------------------///////////
///                                                                       ///
///                    Diffusion-Limited Aggregation                      ///
///                                                                       ///
/////////// ------------------------------------------------------///////////

use rand::Rng;

use super::MapBuilder;
use super::Map;
use super::map::TileType;


pub struct DiffusionLimitedAggregationBuilder {
    map : Map,
    iterations: i32,
}

impl<'a> MapBuilder<'a> for DiffusionLimitedAggregationBuilder {
    fn build(&mut self) {
        self.clear();
        self.seed();
    }

    fn iterate(&mut self) {
        let desired_tiles = 20;
        if self.iterations < 16 {
            self.iterate(desired_tiles);
            self.iterations += 1;
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn update_map_data(&self, map_data: &mut Vec<(&'a str, String)>) {
        map_data.clear();
        let num_walls = self.num_walls();
        map_data.push(("Name", format!("Diffusion-Limited Aggregation")));
        map_data.push(("Max Iterations", format!("{}", 16)));
        map_data.push(("Iteration", format!("{}", self.iterations)));
        map_data.push(("Total Number of Walls", format!("{}", num_walls)));
        map_data.push(("Total Empty Space", format!("{}", 1600 - num_walls)));
        map_data.push(("% Occupied", format!("{}", num_walls as f32 / 1600 as f32)));
    }

    fn notes(&self) -> &str {
        "The intense generation time is mostly spent finding a suitable point on the bounding edge of the map, not its unique aggregation. This specific application is more so focused on 'beauty' rather than 'performance'."
    }
}

impl DiffusionLimitedAggregationBuilder {
    pub fn new() -> DiffusionLimitedAggregationBuilder {
        DiffusionLimitedAggregationBuilder {
            map : Map::new(),
            iterations: 0,
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

    pub fn iterate(&mut self, desired_tiles: i32) {
        let mut x: usize;
        let mut y: usize;
        let mut rng = rand::thread_rng();
        let mut total_tiles = 0;

        while total_tiles < desired_tiles {
            // generates a random point on the edge of the map
            loop {
                x = rng.gen_range(0..self.map.width);
                y = rng.gen_range(0..self.map.height);
                if (self.map.get(x, y) == Some(&TileType::Wall)) && (x == 1 || y == 1 || x == self.map.width - 2 || y == self.map.height - 2) {
                    break;
                }
            }

            loop {
                // check bounds (Doesn't seem to catch the bottom edge?)
                if x == 0 || y == 0 || x == self.map.width - 1 || y == self.map.height - 1 {
                    break;
                }

                // store last position
                let last_pos = (x, y);
    
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
                    // if the drunk hits a floor, the previous tile becomes a floor
                    if tile == &TileType::Floor {
                        self.map.set(TileType::Floor, last_pos.0, last_pos.1);
                        total_tiles += 1;
                        break;
                    }
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