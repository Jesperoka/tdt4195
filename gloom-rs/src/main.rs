// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod scene_geometry;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 800; // 600

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
const fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
const fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
const fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>) -> u32 {
    // Named constants for clarity 
    const NUM_TO_MAKE: i32 = 1;
    const INDEX_ATTR_0: u32 = 0;
    const INDEX_ATTR_1: u32 = 1;
    const SIZE_ATTR_0: i32 = 3;
    const SIZE_ATTR_1: i32 = 4;
    const TYPE_ATTR: u32 = gl::FLOAT;
    const NORMALIZED_ATTR: u8 = gl::FALSE;
    const STRIDE_ATTR_0: i32 = SIZE_ATTR_0 * size_of::<f32>(); 
    const STRIDE_ATTR_1: i32 = SIZE_ATTR_1 * size_of::<f32>(); 
    const OFFSET_ATTR_0: *const c_void = offset::<f32>(0);
    const OFFSET_ATTR_1: *const c_void = offset::<f32>(0);

    let mut vao_id = 0;
    let mut vbo_id_0 = 0;
    let mut vbo_id_1 = 0;
    let mut ibo_id = 0;

    // Generate and bind the Vertex Array Object
    gl::GenVertexArrays(NUM_TO_MAKE, &mut vao_id);
    gl::BindVertexArray(vao_id);

    // Generate and bind the first Vertex Buffer Object
    gl::GenBuffers(NUM_TO_MAKE, &mut vbo_id_0);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_0);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(&vertices),
        pointer_to_array(&vertices),
        gl::STATIC_DRAW,
    );

    // Configure the first Vertex Attribute Poiner, vertices are structured as [x, y, z]
    gl::VertexAttribPointer(INDEX_ATTR_0, SIZE_ATTR_0, TYPE_ATTR, NORMALIZED_ATTR, STRIDE_ATTR_0, OFFSET_ATTR_0);  
    gl::EnableVertexAttribArray(INDEX_ATTR_0);

    // Generate and bind the second Vertex Buffer Object
    gl::GenBuffers(NUM_TO_MAKE, &mut vbo_id_1);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id_1);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(&colors),
        pointer_to_array(&colors),
        gl::STATIC_DRAW,
    );

    // Configure the second Vertex Attribute Poiner, colors are structured as [r, g, b, a]
    gl::VertexAttribPointer(INDEX_ATTR_1, SIZE_ATTR_1, TYPE_ATTR, NORMALIZED_ATTR, STRIDE_ATTR_1, OFFSET_ATTR_1);  
    gl::EnableVertexAttribArray(INDEX_ATTR_1);

    // Generate and bind the Index Buffer Object
    gl::GenBuffers(NUM_TO_MAKE, &mut ibo_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(&indices),
        pointer_to_array(&indices),
        gl::STATIC_DRAW,
    );

    return vao_id;
}


fn main() {
    print!("Setting window size to 800x800, fullscreening window crashes with this setup");
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(CursorGrabMode::Locked).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            //gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here
        let _my_vao = unsafe { 
            create_vao(
                &scene_geometry::VERTICES.to_vec(), 
                &scene_geometry::INDICES.to_vec(),
                &scene_geometry::COLORS.to_vec() 
        )};

        // == // Set up your shaders here
        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.frag")
                .attach_file("./shaders/simple.vert")
                .link()
        };

        unsafe { simple_shader.activate(); }

        let time_location = unsafe { 
            simple_shader.get_uniform_location("time")
        };
        let homography_location = unsafe {
            simple_shader.get_uniform_location("homography")
        };
        let resolution_location = unsafe {
            simple_shader.get_uniform_location("resolution")
        };

        // Initialize variables for camera control
        let fovy: f32 = 1.22;
        let near: f32 = 0.1;
        let far: f32 = 1000.0;
        let mut perspective: glm::Mat4 = glm::perspective_lh_no(window_aspect_ratio, fovy, near, far);

        let mut major_axis_translation: glm::Mat4 = glm::Mat4::identity();
        let mut angle_axis_rotation: glm::Mat4 = glm::Mat4::identity();

        let mut width = INITIAL_SCREEN_W;
        let mut height = INITIAL_SCREEN_H;

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;

                    perspective = glm::perspective_lh_no(window_aspect_ratio, fovy, near, far);
                    width = new_size.0;
                    height = new_size.1;

                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // 1. Get some approximated angle from 2D mouse movement vector assumed to be arc in 3D
                // 2. Define corresponding 3D axis to rotate about. The axis is perpendicular to the mouse movement direction
                // 3. Use angle-axis representation to generate rotation matrix and update
                
                let angle = 0.001*glm::length(&glm::vec2(delta.0, delta.1));
                let rotation_axis = glm::normalize(&glm::vec3(-delta.1, -delta.0, 0.0));
                angle_axis_rotation = glm::rotation(angle, &rotation_axis)*angle_axis_rotation;

                *delta = (0.0, 0.0); // reset when done
            }

            // Handle keyboard input
            const SPEED: f32 = 1.0;
            if let Ok(keys) = pressed_keys.lock() {
                
                let mut dx: f32 = 0.0f32;
                let mut dy: f32 = 0.0f32;
                let mut dz: f32 = 0.0f32;

                for key in keys.iter() {
                    let (_dx, _dy, _dz) = match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

                        VirtualKeyCode::W => (0.0f32, 0.0f32, -delta_time*SPEED),
                        VirtualKeyCode::A => (delta_time*SPEED, 0.0f32, 0.0f32),
                        VirtualKeyCode::S => (0.0f32, 0.0f32, delta_time*SPEED),
                        VirtualKeyCode::D => (-delta_time*SPEED, 0.0f32, 0.0f32),
                        VirtualKeyCode::Space => (0.0f32, -delta_time*SPEED, 0.0f32),
                        VirtualKeyCode::LShift => (0.0f32, delta_time*SPEED, 0.0f32),
                        // default handler:
                        _ => (0.0f32, 0.0f32, 0.0f32),
                    };
                    dx += _dx; dy += _dy; dz += _dz;
                }
                let translation_vector = glm::inverse(&angle_axis_rotation) * glm::vec4(dx, dy, dz, 1.0);

                major_axis_translation[(0, 3)] += translation_vector.x;
                major_axis_translation[(1, 3)] += translation_vector.y;
                major_axis_translation[(2, 3)] += translation_vector.z;
            }

            let translation: glm::Mat4 = glm::Mat4::from([
                                                         [1.0, 0.0, 0.0, 0.0],
                                                         [0.0, 1.0, 0.0, 0.0],
                                                         [0.0, 0.0, 1.0, 2.5],
                                                         [0.0, 0.0, 0.0, 1.0]]).transpose();

            // == // Please compute camera transforms here (exercise 2 & 3)
            let homography: glm::Mat4 = perspective * angle_axis_rotation * major_axis_translation * translation;

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // == // Issue the necessary gl:: commands to draw your scene here
                simple_shader.activate();

                gl::Uniform1f(time_location, elapsed);
                gl::UniformMatrix4fv(homography_location, 1, gl::FALSE, homography.as_ptr());
                gl::Uniform2f(resolution_location, width as f32, height as f32);
                
                gl::DrawElements(gl::TRIANGLES, scene_geometry::INDICES.len() as i32, gl::UNSIGNED_INT, ptr::null());
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // FOR CUSTOM MOUSE EVENT HANDLING SINCE DeviceEvent DOESNT WORK WITH WSL2 FOR ME
    // ------------------------------------------------------------------------------
    let mut prev_cursor_pos: Option<glutin::dpi::PhysicalPosition<f64>> = None;
    let mut mouse_button_down = false;
    let mut reset_cursor_pos = false;
    // ------------------------------------------------------------------------------

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }

            // FOR CUSTOM MOUSE EVENT HANDLING SINCE DeviceEvent DOESNT WORK WITH WSL2 FOR ME
            // ------------------------------------------------------------------------------
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                
                if reset_cursor_pos {
                    prev_cursor_pos = Some(position); 
                    reset_cursor_pos = false;
                }

                if mouse_button_down {
                    if let Some(prev_position) = prev_cursor_pos {
                        let delta_x = position.x - prev_position.x;
                        let delta_y = position.y - prev_position.y;

                        // Only accumulate mouse movement when mouse button is down
                        if let Ok(mut delta) = arc_mouse_delta.lock() {
                            *delta = (delta.0 + delta_x as f32, delta.1 + delta_y as f32);
                        }
                    }
                    prev_cursor_pos = Some(position);
                }
            }
            Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, .. } => {

                if button == glutin::event::MouseButton::Left {

                    match state {
                        Pressed => {
                            mouse_button_down = true;
                            reset_cursor_pos = true;
                        },
                        Released => {
                            mouse_button_down = false;
                        }
                    };
                }
            }
            // ------------------------------------------------------------------------------
            
            // Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
            //     // Accumulate mouse movement
            //     if let Ok(mut position) = arc_mouse_delta.lock() {
            //         *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
            //         print!("position.0 {}, position.1 {}", position.0, position.1)
            //     }
            // }
            _ => { }
        }
    });
}
