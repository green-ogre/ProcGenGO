use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, TextureCreator, Canvas};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

static SCREEN_WIDTH: u32 = 1000;
static SCREEN_HEIGHT: u32 = 800;

mod map;

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


fn render_map(canvas: &mut Canvas<Window>, font: &Font, texture_creator: &TextureCreator<WindowContext>, map: &map::Map)  -> Result<(), String> {
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


fn run(font_path: &Path, mut map: map::Map) -> Result<(), String> {
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
                            map::generate(&mut map);
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

fn main() -> Result<(), String> {
    
    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());

    let mut map = map::new_map();
    map::generate(&mut map);

    let path: &Path = Path::new("fonts/joystix monospace.ttf");
    run(path, map)?;

    Ok(())
}
