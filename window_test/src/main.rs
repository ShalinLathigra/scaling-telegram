extern crate sdl2;
extern crate gl;

pub mod render_gl;
pub mod shapes;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point
};
use std::{
    env, time::Duration
};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("No input args provided".to_string());
    }

    let mut width: u32 = 800;
    let mut height: u32 = 600;

    if args.len() >= 4 {
        match args[2].parse::<u32>() {
            Ok(w) => width = w,
            _ => return Err("Failed to parse width string (arg2) into int".to_string())
        };
        match args[3].parse::<u32>() {
            Ok(h) => height = h,
            _ => return Err("Failed to parse height string (arg3) into int".to_string())
        };
    }
    match args[1].as_str() {
        "sdl" => return sdl_example(),
        "gl" => return gl_example(width, height),
        _ => return Err("Unsupported Input Arg".to_string())
    }
}

fn gl_example(width: u32, height: u32) -> Result<(), String> {
    // initialize context + video subsystem
    // \? operator automatically passes errors up the pipe, also allowing us to directly access the result contents
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    // Define window + canvas
    let window = video_subsystem
        .window("My Rust Demo", width,height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // Pull out the opengl context from the window
    let _gl_context = window.gl_create_context()?;
    // gl "forwards opengl function calls to the driver"
    // to do this, it needs to be initialized with some function it can use to load in function pointers as a string
    // |s| is a single arg closure (a string slice)
    // gl_get_proc_address takes in a process name, Gets the pointer to the named OpenGL function.
    // cast it to a C void pointer.
    let gl =
        gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    // find source of events.
    let mut event_pump = sdl_context.event_pump()?;

    // init program and init shaders
    // include_str! causes the text inside of the file to be compiled into the program!!
    // Means I don't need to worry about parsing it out too!
    use std::ffi::CString;

    // Rust UTF-8 strings *can* contain 0 in the middle while still being valid.
    // Not an expected case so we are fine with this.
    let vert_shader1 = render_gl::Shader::from_vert_source(
        &gl,
        &CString::new(include_str!("shaders/gl_triangle.vert")).unwrap()
    ).unwrap();
    let frag_shader1 = render_gl::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("shaders/gl_triangle.frag")).unwrap()
    ).unwrap();

    let vert_shader2 = render_gl::Shader::from_vert_source(
        &gl,
        &CString::new(include_str!("shaders/my_triangle.vert")).unwrap()
    ).unwrap();
    let frag_shader2 = render_gl::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("shaders/my_triangle.frag")).unwrap()
    ).unwrap();

    // initialize program from shaders
    let shader_program1 = render_gl::Program::from_shaders(
        &gl, &[vert_shader1, frag_shader1])?;
    let shader_program2 = render_gl::Program::from_shaders(
        &gl, &[vert_shader2, frag_shader2])?;

    // define verts + shapes
    let triangle1_verts: Vec<f32> = vec![
        -0.9, -0.5 + 0.25, 0.0, 1.0, 0.0, 0.0,
        0.1, -0.5 + 0.25, 0.0, 0.0, 1.0, 0.0,
        -0.4, 0.5 + 0.25, 0.0, 0.0, 0.0, 1.0,
    ];
    let triangle2_verts: Vec<f32> = vec![
        -0.9 + 0.8, -0.5 - 0.25, 0.0, 1.0, 0.0, 0.0,
        0.1 + 0.8, -0.5 - 0.25, 0.0, 0.0, 1.0, 0.0,
        -0.4 + 0.8, 0.5 - 0.25, 0.0, 0.0, 0.0, 1.0,
    ];

    let triangle1: shapes::Triangle = shapes::Triangle::from_array(
        &gl,&triangle1_verts)?;
    let triangle2: shapes::Triangle = shapes::Triangle::from_array(
        &gl,&triangle2_verts)?;

    // initial state of viewport shared window
    unsafe {
        gl.Viewport(0,0, width as i32, height as i32);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

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
                _ => {}
            }
        }
        // Draw

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangles
        shader_program1.set_used();
        unsafe {
            gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl.BindVertexArray(triangle1.vao());
            // equivalent of calling the individual subroutines on each bound vertex array
            gl.DrawArrays(
                gl::TRIANGLES,
                0, // index in the enabled array(s)
                3 // # indices to be rendered
            );
        }

        shader_program2.set_used();
        unsafe {
            gl.BindVertexArray(triangle2.vao());
            gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            // equivalent of calling the individual subroutines on each bound vertex array
            gl.DrawArrays(
                gl::TRIANGLES,
                0, // index in the enabled array(s)
                3 // # indices to be rendered
            );
        }
        window.gl_swap_window();
        // wait for next frame
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}

fn sdl_example() -> Result<(), String> {
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

    let mut mouse_point = Point::new(0, 0);

    // find source of events.
    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err( |e| e.to_string())?;

    canvas.set_scale(SCALE_FACTOR as f32, SCALE_FACTOR as f32)?;

    // clear the canvas initially
    canvas.set_draw_color(Color::RGB(0, 127, 127));
    canvas.clear();
    canvas.present();
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
    Ok(())
}