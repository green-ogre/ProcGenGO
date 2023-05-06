use std::path::Path;
use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, TextureCreator, Canvas};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

static SCREEN_WIDTH: u32 = 1000;
static SCREEN_HEIGHT: u32 = 800;

pub mod map;
use map::Map;


// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}


fn render_map(canvas: &mut Canvas<Window>, font: &Font, texture_creator: &TextureCreator<WindowContext>, map: &Map)  -> Result<(), String> {
    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render(&format!("{map}"))
        .blended_wrapped(Color::RGBA(255, 255, 255, 255), 0)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(84, 73, 73, 255));
    canvas.clear();

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(
        width,
        height,
        SCREEN_WIDTH - padding,
        SCREEN_HEIGHT - padding,
    );

    canvas.copy(&texture, None, Some(target))?;
    canvas.present();

    Ok(())
}


fn run(font_path: &Path, map: &mut Map, textures: map::TexturePack) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsys
        .window("ProcGenGO", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context.load_font(font_path, 18)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    render_map(&mut canvas, &font, &texture_creator, &map)?;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit{ .. } => break 'mainloop,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::R => {
                            heuristic(map, &textures);
                            render_map(&mut canvas, &font, &texture_creator, &map)?;
                        },
                        Keycode::Escape => break 'mainloop,
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    Ok(())
}


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
        let m1_x = i32::clone(m1.get(0).expect("m1_x"));
        let m1_y = i32::clone(m1.get(1).expect("m1_y"));
        let m2_x = i32::clone(m2.get(0).expect("m2_x"));
        let m2_y = i32::clone(m2.get(1).expect("m2_y"));

        // Center of room Cordinates
        print!("{m1_x}, {m1_y} : {m2_x}, {m2_y}\n");

        let height: i32;
        let mut start: i32;
        let destination: i32;

        // Draw vertical component
        if m1_y > m2_y {
            for h in 0..(m1_y - m2_y) {
                map.get_mut_cord(m1_x as usize, (h + m2_y) as usize).set_material(&textures.path);
            }
            height = m2_y
        }
        else if m1_y < m2_y {
            for h in 0..(m2_y - m1_y) {
                map.get_mut_cord(m2_x as usize, (h + m1_y) as usize).set_material(&textures.path);
            }
            height = m1_y;
        }
        else {
            height = m1_y;
        }

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


fn main() -> Result<(), String> {
    
    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());

    let mut map = map::new_map();

    let textures = map::TexturePack {
        empty: String::from("."),
        room: String::from("X"),
        path: String::from("X")
    };

    heuristic(&mut map, &textures);

    let path: &Path = Path::new("fonts/joystix monospace.ttf");
    run(path, &mut map, textures)?;

    Ok(())
}
