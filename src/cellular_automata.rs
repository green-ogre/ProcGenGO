/////////// ------------------------------------------------------///////////
///                                                                       ///
///                        Cellular Automata                              ///
///                                                                       ///
/////////// ------------------------------------------------------///////////


pub mod map {
    use arrayvec::ArrayVec;
    
    pub struct Map {
        pub height: usize,
        pub width: usize,
        pub render: ArrayVec<bool, 1600>,
        pub iterations: i32
    }

    impl Map {
        pub fn new() -> Self {
            let mut map = Map {
                height: 40,
                width: 40,
                render: ArrayVec::<bool, 1600>::new(),
                iterations: 0
            };
        
            scramble(&mut map);
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
    
    pub fn scramble(map: &mut Map) {
        map.render.clear();
        map.iterations = 0;

        for _r in 0..map.height {
            for _c in 0..map.width {
                map.render.push(fastrand::i32(0..2) == 0);
            }
        }
    }
}


use arrayvec::ArrayVec;


pub fn iterate(map: &mut map::Map) {
    let mut new_render = ArrayVec::<bool, 1600>::new();
    for y in 0..map.height {
        for x in 0..map.width {
            let num_neighbors = count_neighbors(&map, x, y);

            if num_neighbors == -1 {
                new_render.push(map.get_pos(x, y).clone());
            }

            if num_neighbors == 0 || num_neighbors >= 5 {
                new_render.push(true);
            }
            else if 1 <= num_neighbors && num_neighbors <= 4 {
                new_render.push(false);
            }
        }
    }
    map.render = new_render;
    map.iterations += 1;
}

fn count_neighbors(map: &map::Map, x: usize, y: usize) -> i32 {
    if x == 0 || y == 0 || x == map.width - 1 || y == map.height - 1 {
        return -1;
    }

    let mut num_neighbors = 0;

    // X . .
    // . . .
    // . . .
    num_neighbors += has_neighbor(&map, x - 1, y - 1);

    // . . .
    // X . .
    // . . .
    num_neighbors += has_neighbor(&map, x - 1, y);

    // . . .
    // . . .
    // X . .
    num_neighbors += has_neighbor(&map, x - 1, y + 1);

    // . . .
    // . . X
    // . . .
    num_neighbors += has_neighbor(&map, x + 1, y);

    // . . .
    // . . .
    // . X .
    num_neighbors += has_neighbor(&map, x, y + 1);

    // . X .
    // . . .
    // . . .
    num_neighbors += has_neighbor(&map, x, y - 1);

    // . . X
    // . . .
    // . . .
    num_neighbors += has_neighbor(&map, x + 1, y - 1);

    // . . .
    // . . .
    // . . X
    num_neighbors += has_neighbor(&map, x + 1, y + 1);

    num_neighbors
}

fn has_neighbor(map: &map::Map, x: usize, y: usize) -> i32 {
    let result = map.render.get(map.width * y + x);
    match result {
        Some(r) => match r {
            true => 1,
            false => 0
        },
        None => 0
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
    data.push(("Iterations", format!("{}", map.iterations)));
    let num_walls = num_walls(&map);
    data.push(("Total Number of Walls", format!("{}", num_walls)));
    data.push(("Total Empty Space", format!("{}", 1600 - num_walls)));
    data.push(("% Occupied", format!("{}", num_walls as f32 / 1600 as f32)));
}