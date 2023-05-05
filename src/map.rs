
use rand::Rng;
use arrayvec::ArrayVec;


struct TexturePack {
    empty: String,
    room: String,
    path: String
}


struct Cord {
    x: i32,
    y: i32,
    material: String
}


struct Room {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    middle: Vec<i32>
}

impl Room {
    fn push_to_map(&self, map: &mut ArrayVec<Cord, 1600>, map_width: i32, room_material: String) {
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                if let Some(cord) = map.get_mut((map_width * (self.y + y as i32) + (self.x + x as i32)) as usize) {
                    *cord = Cord {
                        x: self.x + x as i32,
                        y: self.y + y as i32,
                        material: String::from(&room_material)
                    }
                }
            }
        }
    }
}


pub struct Map {
    height: usize,
    width: usize,
    textures: TexturePack,
    r_list: Vec<Room>,
    c_map: ArrayVec<Cord, 1600>,
    max_room_size: i32,
    min_room_size: i32,
    max_rooms: i32,
    min_rooms: i32
}

impl Map {
    fn init_empty_map(&mut self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let cord = Cord {
                    x: c as i32,
                    y: r as i32,
                    material: String::from(&self.textures.empty)
                };
                self.c_map.push(cord);
            }
        }
    }

    fn regenerate(&mut self) {
        // Clear old data
        self.r_list.clear();
        for r in 0..self.height {
            for c in 0..self.width {
                self.c_map.get_mut(self.width * r + c)
                    .expect("error retrieving mut cord")
                    .material = String::from(&self.textures.empty);
            }
        }

        let mut rng = rand::thread_rng();
        let mut room_count = rng.gen_range(self.min_rooms..self.max_rooms);

        while room_count > 0 {
            // Size
            let room_width = rng.gen_range(self.min_room_size..self.max_room_size);
            let room_height = rng.gen_range(self.min_room_size..self.max_room_size);

            // Random top left origin
            let mut x = rng.gen_range(1..self.width as i32 - 1);
            let mut y = rng.gen_range(1..self.height as i32 - 1);

            // Verify map bounds
            while x + room_width + 1 > self.width as i32 || y + room_height + 1 > self.height as i32 {
                x = rng.gen_range(1..self.width as i32 - 1);
                y = rng.gen_range(1..self.height as i32 - 1);
            }

            // Verify no overlap
            let mut valid = true;
            for r in (y - 1)..(y + room_height + 1) {
                for c in (x - 1)..(x + room_width + 1) {
                    if self.c_map.get(((self.width as i32) * (r) + (c)) as usize).expect("1").material.eq(&self.textures.room) {
                        valid = false;
                        break;
                    }
                }
            }

            // Add room into map if valid
            if valid {
                let new_room = Room {
                    x, 
                    y, 
                    width: room_width as u32, 
                    height: room_height as u32,
                    middle: vec![x + room_width / 2, y + room_height / 2]
                };

                new_room.push_to_map(&mut self.c_map, self.width as i32, String::from(&self.textures.room));
                self.r_list.push(new_room);
                room_count -= 1;
            }
        }
    }

    fn draw_paths(&mut self) {
        for i in 0..(&self.r_list.len() - 1) {
            let m1 = &self.r_list.get(i).expect("room retrieve error").middle;
            let m2 = &self.r_list.get(i + 1).expect("room retrieve error").middle;
            let m1_x = m1.get(0).expect("m1_x");
            let m1_y = m1.get(1).expect("m1_y");
            let m2_x = m2.get(0).expect("m2_x");
            let m2_y = m2.get(1).expect("m2_y");

            // Center of room Cordinates
            print!("{m1_x}, {m1_y} : {m2_x}, {m2_y}\n");
    
            let height: i32;
            let mut start: i32;
            let destination: i32;
    
            // Draw vertical component
            if m1_y > m2_y {
                for h in 0..(m1_y - m2_y) {
                    self.c_map.get_mut(((h + m2_y) * self.width as i32 + m1_x) as usize).expect("path1").material = String::from(&self.textures.path);
                }
                height = i32::clone(m2_y);
            }
            else if m1_y < m2_y {
                for h in 0..(m2_y - m1_y) {
                    self.c_map.get_mut(((h + m1_y) * self.width as i32 + m2_x) as usize).expect("path2").material = String::from(&self.textures.path);
                }
                height = i32::clone(m1_y);
            }
            else {
                height = i32::clone(m1_y);
            }
    
            // Find start and destination based on direction (start is always < destination)
            if m2_x - m1_x > 0 {
                start = i32::clone(m1_x);
                destination = i32::clone(m2_x);
            } else {
                start = i32::clone(m2_x);
                destination = i32::clone(m1_x);
            }
    
            // Draw horizontal component
            while start != destination {
                self.c_map.get_mut((height * self.width as i32 + start) as usize).expect("path2").material = String::from(&self.textures.path);
                start += 1;
            }
        }
    }
}


// Custom formatting for a print! call
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let cord = self.c_map.get((self.width * r + c) as usize).expect("2");
                write!(f, "{} ", &cord.material)?;
                // print!("{} ", &cord.material);
            }
            f.write_str("\n")?;
            // println!();
        }
        Ok(())
    }
}


pub fn new_map() -> Map {
    let t = TexturePack {
        empty: String::from("."),
        room: String::from("X"),
        path: String::from("X")
    };

    let width: usize = 40;
    let height: usize = 40;


    let mut map = Map {
        height,
        width,
        textures: t,
        r_list: Vec::<Room>::new(),
        c_map: ArrayVec::<Cord, 1600>::new(),
        max_room_size: 8,
        min_room_size: 4,
        max_rooms: 10,
        min_rooms: 5
    };

    map.init_empty_map();
    map
}

pub fn generate(map: &mut Map) {
    map.regenerate();
    map.draw_paths();
}
