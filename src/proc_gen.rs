pub mod map;

use map::Map;
use rand::Rng;
use std::cmp::Ordering;


/////////// --------------------------------------------------------------------------
/// 
///                           Heuristic
/// 
/////////// --------------------------------------------------------------------------


pub fn heuristic(map: &mut Map, textures: &map::TexturePack) {
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


