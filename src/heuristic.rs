/////////// ------------------------------------------------------///////////
///                                                                       ///
///                             Heuristic                                 ///
///                                                                       ///
/////////// ------------------------------------------------------///////////

pub mod map {
    use arrayvec::ArrayVec;

    pub struct Room {
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32,
        pub middle: Vec<i32>
    }
    
    pub struct Map {
        pub height: usize,
        pub width: usize,
        r_list: Vec<Room>,
        c_map: ArrayVec<Cord, 1600>,
        pub max_room_size: i32,
        pub min_room_size: i32,
        pub max_rooms: i32,
        pub min_rooms: i32
    }
    
    pub struct Cord {
        //x: i32,
        //y: i32,
        pub material: char
    }
    
    pub struct TexturePack {
        pub empty: char,
        pub room: char,
        pub path: char
    }
    
    impl TexturePack {
        pub fn new() -> Self {
            TexturePack {
                empty: ' ',
                room: '■',
                path: '■'
            }
        }
    }
    
    
    impl Room {
        pub fn push_to_map(&self, map: &mut Map, material: &char) {
            for y in 0..self.height - 1 {
                for x in 0..self.width - 1 {
                    map.get_mut_cord((self.x + x) as usize, (self.y + y) as usize).set_material(material);
                }
            }
        }
    }
    
    impl Cord {
        pub fn set_material(&mut self, material: &char) {
            self.material = *material;
        }
    }
    
    impl Map {
        pub fn new() -> Self {
            let mut map = Map {
                height: 40,
                width: 40,
                r_list: Vec::<Room>::new(),
                c_map: ArrayVec::<Cord, 1600>::new(),
                max_room_size: 12,
                min_room_size: 4,
                max_rooms: 14,
                min_rooms: 8
            };
        
            init_empty_map(&mut map, &' ');
            map
        }
    
        pub fn get_mut_cord(&mut self, x: usize, y: usize) -> &mut Cord {
            self.c_map.get_mut(self.width * y + x).expect("Could not find cord")
        }
    
        pub fn get_cord(&self, x: usize, y: usize) -> &Cord {
            self.c_map.get(self.width * y + x).expect("Could not find cord")
        }
    
        pub fn push_room(&mut self, room: Room) {
            self.r_list.push(room);
        }
    
        pub fn get_mut_room(&mut self, i: i32) -> &mut Room {
            self.r_list.get_mut(i as usize).expect("Could not find room")
        }
    
        pub fn get_room(&self, i: i32) -> &Room {
            self.r_list.get(i as usize).expect("Could not find room")
        }
    
        pub fn num_rooms(&self) -> i32 {
            self.r_list.len() as i32
        }
    
        pub fn clear(&mut self, material: &char) {
            self.r_list.clear();
            for y in 0..self.height {
                for x in 0..self.width {
                    self.get_mut_cord(x, y).set_material(material);
                }
            }
        }
    
        pub fn room_sa(&self) -> u64 {
            let mut sa = 0;
            for room in &self.r_list {
                sa += room.width * room.height;
            }
            sa as u64
        }
    }
    
    impl std::fmt::Display for Map {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            for y in 0..self.height {
                for x in 0..self.width {
                    let cord = self.get_cord(x, y);
                    write!(f, "{} ", &cord.material)?;
                    // print!("{} ", &cord.material);
                }
                f.write_str("\n")?;
                // println!();
            }
            Ok(())
        }
    }
    
    
    fn init_empty_map(map: &mut Map, material: &char) {
        for _r in 0..map.height {
            for _c in 0..map.width {
                let cord = Cord {
                    //x: c as i32,
                    //y: r as i32,
                    material: *material
                };
                map.c_map.push(cord);
            }
        }
    }
}


use map::Map;
use rand::Rng;
use std::cmp::Ordering;


pub fn run(map: &mut Map, textures: &map::TexturePack) {
    // Clear old data
    map.clear(&textures.empty);

    let mut rng = rand::thread_rng();
    let mut room_count = rng.gen_range(map.min_rooms..map.max_rooms);

    while room_count > 0 {
        // Size
        let room_width = rng.gen_range(map.min_room_size..map.max_room_size);
        let room_height = rng.gen_range(map.min_room_size..map.max_room_size);

        // Random top left origin
        let mut x = rng.gen_range(1..map.width as i32 - 1);
        let mut y = rng.gen_range(1..map.height as i32 - 1);

        // Verify map bounds
        while x + room_width + 1 > map.width as i32 || y + room_height + 1 > map.height as i32 {
            x = rng.gen_range(1..map.width as i32 - 1);
            y = rng.gen_range(1..map.height as i32 - 1);
        }

        // Verify no overlap
        let mut valid = true;
        for y in (y - 1)..(y + room_height + 1) {
            for x in (x - 1)..(x + room_width + 1) {
                if map.get_mut_cord(x as usize, y as usize).material.eq(&textures.room) {
                    valid = false;
                    break;
                }
            }
        }

        // Add room into map if valid
        if valid {
            let new_room = map::Room {
                x, 
                y, 
                width: room_width, 
                height: room_height,
                middle: vec![x + room_width / 2, y + room_height / 2]
            };

            new_room.push_to_map(map, &textures.room);
            map.push_room(new_room);
            room_count -= 1;
        }
    }
    draw_paths(map, textures);
}

fn draw_paths(map: &mut Map, textures: &map::TexturePack) {
    for i in 0..(map.num_rooms() - 1) {
        let m1 = &map.get_room(i).middle;
        let m2 = &map.get_room(i + 1).middle;
        let m1_x = i32::clone(m1.first().expect("m1_x"));
        let m1_y = i32::clone(m1.last().expect("m1_y"));
        let m2_x = i32::clone(m2.first().expect("m2_x"));
        let m2_y = i32::clone(m2.last().expect("m2_y"));

        // Center of room Cordinates
        //print!("{m1_x}, {m1_y} : {m2_x}, {m2_y}\n");

        let mut start: i32;
        let destination: i32;

        // Draw vertical component
        let height = match m1_y.cmp(&m2_y) {
            Ordering::Greater => {
                for h in 0..(m1_y - m2_y) {
                    map.get_mut_cord(m1_x as usize, (h + m2_y) as usize).set_material(&textures.path);
                }
                m2_y
            },
            Ordering::Less => {
                for h in 0..(m2_y - m1_y) {
                    map.get_mut_cord(m2_x as usize, (h + m1_y) as usize).set_material(&textures.path);
                }
                m1_y
            },
            Ordering::Equal => {
                m1_y
            }
        };

        // Find start and destination based on direction (start is always < destination)
        if m2_x - m1_x > 0 {
            start = m1_x;
            destination = m2_x;
        } else {
            start = m2_x;
            destination = m1_x;
        }

        // Draw horizontal component
        while start != destination {
            map.get_mut_cord(start as usize, height as usize).set_material(&textures.path);
            start += 1;
        }
    }
}

pub fn update_data_list(map: &map::Map, data: &mut Vec<(&str, String)>) {
    data.clear();
    data.push(("Number of Rooms", format!("{}", map.num_rooms())));
    data.push(("Total Room Surface Area", format!("{}", map.room_sa())));
}
