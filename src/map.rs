
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
    material: String,
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

    pub fn regenerate(&mut self) {
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
                    middle: vec![x + room_width / 2, y + room_height / 2],
                    material: String::from(&self.textures.room)
                };

                new_room.push_to_map(&mut self.c_map, self.width as i32, String::from(&self.textures.room));
                self.r_list.push(new_room);
                room_count -= 1;
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


fn draw_paths(map: &mut Map) {
    for i in 0..(map.r_list.len() - 1) {
        let m1 = map.r_list.get(i).middle;
        let m2 = map.r_list.get(i + 1).middle;

        if m1.get(1) > m2.get(1) {
            for h in 0..(m1.get(1) - m2.get(1)) {
                map.c_map.get_mut(h + m2.get(1))
            }
        }
    }
}


for i in range(len(room_list) - 1):
        m1 = room_list[i].middle()
        m2 = room_list[i + 1].middle()

        #Draw vertical component
        if m1[1] > m2[1]:
            for h in range(m1[1] - m2[1]):
                map[h + m2[1]][m1[0]].change_material(PATH_MATERIAL)
            height = m2[1]
        elif m1[1] < m2[1]:
            for h in range(m2[1] - m1[1]):
                map[h + m1[1]][m2[0]].change_material(PATH_MATERIAL)
            height = m1[1]
        else:
            height = m1[1]
        
        #Determine direction towards destination
        if m2[0] - m1[0] > 0:
            start = m1[0]
            destination = m2[0]
        else:
            start = m2[0]
            destination = m1[0]

        #Draw horizontal componenet
        while start != destination:
            map[height][start].change_material(PATH_MATERIAL)
            start += 1


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
        max_room_size: 5,
        min_room_size: 4,
        max_rooms: 4,
        min_rooms: 3
    };

    map.init_empty_map();
    map.regenerate();

    map

}
