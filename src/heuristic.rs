pub fn generate(map: &mut Map) {
    // Clear old data
    map.clear();

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
                if map.get_mut_cord(x, y).material.eq(&self.textures.room) {
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

            new_room.push_to_map(&mut map);
            map.push_room(new_room);
            room_count -= 1;
        }
    }
    draw_paths(map);
}

fn draw_paths(map: &mut Map) {
    for i in 0..(map.num_rooms() - 1) {
        let m1 = map.get_room(i).middle;
        let m2 = map.get_room(i + 1).middle;
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
                map.get_mut_cord(m1_x, h + m2_y).set_material(&map.textures.path);
            }
            height = i32::clone(m2_y);
        }
        else if m1_y < m2_y {
            for h in 0..(m2_y - m1_y) {
                map.get_mut_cord(m2_x, h + m1_y).set_material(&map.textures.path);
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
            map.get_mut_cord(start, height).set_material(&map.textures.path);
            start += 1;
        }
    }
}