/////////// ------------------------------------------------------///////////
///                                                                       ///
///                    Diffusion-Limited Aggregation                      ///
///                                                                       ///
/////////// ------------------------------------------------------///////////


pub mod map {
    use arrayvec::ArrayVec;
    
    pub struct Map {
        pub height: usize,
        pub width: usize,
        pub render: ArrayVec<bool, 1600>,
        pub desired_tiles: i32,
        pub total_tiles: i32,
    }

    impl Map {
        pub fn new() -> Self {
            let mut map = Map {
                height: 40,
                width: 40,
                render: ArrayVec::<bool, 1600>::new(),
                desired_tiles: 500,
                total_tiles: 0,
            };
        
            fill(&mut map);
            map
        }

        pub fn get_pos(&self, x: usize, y: usize) -> &bool {
            self.render.get(self.width * y + x).expect("Could not find position")
        }
    }
    
    impl std::fmt::Display for Map {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            for y in 0..self.height {
                for x in 0..self.width {
                    let render = match self.get_pos(x, y) {
                        true => '■',
                        false => ' '
                    };
                    write!(f, "{} ", render)?;
                    // print!("{} ", &cord.material);
                }
                f.write_str("\n")?;
                // println!();
            }
            Ok(())
        }
    }
    
    pub fn fill(map: &mut Map) {
        map.total_tiles = 0;
        map.render.clear();

        for _r in 0..map.height {
            for _c in 0..map.width {
                map.render.push(true);
            }
        }
    }
}


use rand::Rng;


pub fn run(map: &mut map::Map) {
    map::fill(map);

    let mut rng = rand::thread_rng();
    let mut x: usize = map.width / 2;
    let mut y: usize = map.height / 2;

    // create seed in center
    *map.render.get_mut(y * map.width + x + 1).expect("true") = false;
    *map.render.get_mut((y + 1) * map.width + x + 1).expect("true") = false;
    *map.render.get_mut(y * map.width + x).expect("true") = false;
    *map.render.get_mut((y + 1) * map.width + x).expect("true") = false;

    map.total_tiles += 4;

    while map.total_tiles < map.desired_tiles {
        loop {
            loop {
                x = rng.gen_range(0..map.width);
                y = rng.gen_range(0..map.height);
                if map.render.get(y * map.width + x).expect("found point") == &true {
                    break;
                }
            }

            // store last position
            let last_pos = (x, y);

            // check bounds
            if x == 0 || y == 0 || x == map.width - 1 || y == map.height - 1 {
                break;
            }

            // choose a direction and take a step
            match rng.gen_range(0..4) {
                // Up
                0 => {
                    y -= 1;
                },
                // Down
                1 => {
                    y += 1;
                },
                // Left
                2 => {
                    x -= 1;
                },
                // Right
                _ => {
                    x += 1;
                }
            }

            // check for floor
            if let Some(elem) = map.render.get(y * map.width + x) {
                // if the drunk hits a floor, the previous tile becomes a floor
                if elem == &false {
                    *map.render.get_mut(last_pos.1 * map.width + last_pos.0).expect("true") = false;
                    map.total_tiles += 1;
                    break;
                }
            } 
        }
    }
}

fn num_walls(map: &map::Map) -> i32 {
    let mut num_walls = 0;
    for i in 0..1600 {
        if let Some(elem) = map.render.get(i) {
            if elem.eq(&true) {
                num_walls += 1;
            }
        } 
    }
    num_walls
}

pub fn update_data_list(map: &map::Map, data: &mut Vec<(&str, String)>) {
    data.clear();
    data.push(("Desired Number of Floor Tiles", format!("{}", map.desired_tiles)));
    let num_walls = num_walls(&map);
    data.push(("Total Number of Walls", format!("{}", num_walls)));
    data.push(("Total Empty Space", format!("{}", 1600 - num_walls)));
    data.push(("% Occupied", format!("{}", num_walls as f32 / 1600 as f32)));
}