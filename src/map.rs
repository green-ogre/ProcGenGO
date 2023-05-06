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
    x: i32,
    y: i32,
    pub material: String
}

pub struct TexturePack {
    pub empty: String,
    pub room: String,
    pub path: String
}


impl Room {
    pub fn push_to_map(&self, map: &mut Map, material: &String) {
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                map.get_mut_cord((self.x + x) as usize, (self.y + y) as usize).set_material(material);
            }
        }
    }
}

impl Cord {
    pub fn set_material(&mut self, material: &String) {
        self.material = String::from(material);
    }
}

impl Map {
    pub fn get_mut_cord(&mut self, x: usize, y: usize) -> &mut Cord {
        self.c_map.get_mut(self.width * y + x).expect("Could not find cord")
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

    pub fn clear(&mut self, material: &String) {
        self.r_list.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                self.get_mut_cord(x, y).set_material(material);
            }
        }
    }
}

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


fn init_empty_map(map: &mut Map, material: &String) {
    for r in 0..map.height {
        for c in 0..map.width {
            let cord = Cord {
                x: c as i32,
                y: r as i32,
                material: String::from(material)
            };
            map.c_map.push(cord);
        }
    }
}

pub fn new_map() -> Map {
    let mut map = Map {
        height: 40,
        width: 40,
        r_list: Vec::<Room>::new(),
        c_map: ArrayVec::<Cord, 1600>::new(),
        max_room_size: 8,
        min_room_size: 4,
        max_rooms: 10,
        min_rooms: 5
    };

    init_empty_map(&mut map, &String::from("."));
    map
}