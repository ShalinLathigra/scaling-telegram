extern crate sdl2;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point
};
use std::time::Duration;

fn main() -> Result<(), String>{
    // initialize context + video subsystem
    // \? operator automatically passes errors up the pipe, also allowing us to directly access the result contents
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    const SCALE_FACTOR: i32 = 8;

    // Define window + canvas
    let window = video_subsystem
        .window("My Rust Demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err( |e| e.to_string())?;

    canvas.set_scale(SCALE_FACTOR as f32, SCALE_FACTOR as f32)?;

    let mut mouse_point = Point::new(0, 0);

    // clear the canvas initially
    canvas.set_draw_color(Color::RGB(0, 127, 127));
    canvas.clear();
    canvas.present();

    // find source of events.
    let mut event_pump = sdl_context.event_pump()?;

    // main loop
    'running: loop {
        // event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion {
                    mousestate,
                    x,
                    y,
                    ..
                } => {
                    mouse_point = Point::new(x / SCALE_FACTOR, y / SCALE_FACTOR );
                    println!("Moving mouse in state: {:?} at x {} and y {}", mousestate, x / SCALE_FACTOR, y / SCALE_FACTOR );
                },
                _ => {}
            }
        }

        // Draw
        canvas.set_draw_color(Color::RGB(0, 127, 127));
        canvas.clear();
        // set mouse color based on click status
        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(sdl2::mouse::MouseButton::Left)
        {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas.draw_point(mouse_point)?;
        canvas.present();

        // wait for next frame
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    return Ok(())
}

// first thing, SIN/COS/TAN visualizations