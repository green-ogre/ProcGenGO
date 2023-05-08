/////////// ------------------------------------------------------///////////
///                                                                       ///
///                           Perlin Noise!                               ///
///                                                                       ///
/////////// ------------------------------------------------------///////////


pub mod map {
    use arrayvec::ArrayVec;
    
    pub struct Map {
        pub height: usize,
        pub width: usize,
        pub render: ArrayVec<i32, 1600>,
    }

    impl Map {
        pub fn new() -> Self {
            let mut map = Map {
                height: 40,
                width: 40,
                render: ArrayVec::<i32, 1600>::new(),
            };
        
            fill(&mut map);
            map
        }

        // pub fn get_pos(&self, x: usize, y: usize) -> &bool {
        //     self.render.get(self.width * y + x).expect("Could not find position")
        // }
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
        map.iterations = 0;
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
    
}



pub fn update_data_list(map: &map::Map, data: &mut Vec<(&str, String)>) {
    data.clear();
    data.push(("Steps Per Iteration", format!("{}", map.max_steps)));
    data.push(("Iterations", format!("{}", map.iterations)));
    let num_walls = num_walls(&map);
    data.push(("Total Number of Walls", format!("{}", num_walls)));
    data.push(("Total Empty Space", format!("{}", 1600 - num_walls)));
    data.push(("% Occupied", format!("{}", num_walls as f32 / 1600 as f32)));
}