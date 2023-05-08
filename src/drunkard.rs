/////////// ------------------------------------------------------///////////
///                                                                       ///
///                             Drunkard!                                 ///
///                                                                       ///
/////////// ------------------------------------------------------///////////


pub mod map {
    use arrayvec::ArrayVec;
    
    pub struct Map {
        pub height: usize,
        pub width: usize,
        pub render: ArrayVec<bool, 1600>
    }

    impl Map {
        pub fn new() -> Self {
            let mut map = Map {
                height: 40,
                width: 40,
                render: ArrayVec::<bool, 1600>::new()
            };
        
            fill(&mut map);
            map
        }
    
        pub fn update_map_rows(&self, map_rows: &mut Vec::<String>) {
            map_rows.clear();
            for y in 0..self.height {
                let mut row = String::new();
                // fix alignment with rows 0-9
                if y < 10 {
                    row.push(' ');
                }
    
                for x in 0..self.width {
                    row.push(match self.get_pos(x, y) {
                        true => 'X',
                        false => '.'
                    });
                    row.push(' ');
                }
                map_rows.push(row);
            }
        }

        pub fn get_pos(&self, x: usize, y: usize) -> &bool {
            self.render.get(self.width * y + x).expect("Could not find position")
        }
    }
    
    impl Default for Map {
        fn default() -> Self {
            let mut map = Map {
                height: 40,
                width: 40,
                render: ArrayVec::<bool, 1600>::new()
            };
        
            scramble(&mut map);
            map
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
        map.render.clear();

        for _r in 0..map.height {
            for _c in 0..map.width {
                map.render.push(true);
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
}

fn count_neighbors(map: &map::Map, x: usize, y: usize) -> i32 {
    if x == 0 || y == 0 || x == map.width || y == map.height {
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