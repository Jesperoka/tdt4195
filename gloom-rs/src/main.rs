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
use std::collections::HashMap;
use std::{thread, mem, ptr, os::raw::c_void};
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod scene_geometry;
mod mesh;
mod scene_graph;
mod toolbox;

use glutin::event::{Event, WindowEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 800; // 600

// Get the size of an arbitrary array of numbers measured in bytes
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}
// Get the OpenGL-compatible pointer to an arbitrary array of numbers
const fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}
// Get the size of the given type in bytes
const fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}
// Get an offset in bytes for n units of type T, represented as a relative pointer
const fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, normals: &Vec<f32>, colors: &Vec<f32>, indices: &Vec<u32>) -> u32 {
    const NUM_TO_MAKE: i32 = 1;
    const INDEX_ATTRS: [u32; 3] = [0, 1, 2];
    const SIZE_ATTRS: [i32; 3] = [3, 3, 4];
    const TYPE_ATTR: u32 = gl::FLOAT;
    const NORMALIZED_ATTR: u8 = gl::FALSE;
    const STRIDE_ATTRS: [i32; 3] = [SIZE_ATTRS[0]*size_of::<f32>(), SIZE_ATTRS[1]*size_of::<f32>(), SIZE_ATTRS[2]*size_of::<f32>()];
    const OFFSET_ATTRS: [*const c_void; 3] = [offset::<f32>(0), offset::<f32>(0), offset::<f32>(0)];

    // Instantiate IDs
    let (mut vao_id, mut vbo_id_0, mut vbo_id_1, mut vbo_id_2, mut ibo_id) = (0,0,0,0,0);

    // Function: generate and bind a Vertex Buffer Object
    let gen_and_bind_vbo = |vbo_id: &mut u32, attributes: &Vec<f32>| {
        gl::GenBuffers(NUM_TO_MAKE, vbo_id);
        gl::BindBuffer(gl::ARRAY_BUFFER, *vbo_id);
        gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(&attributes), pointer_to_array(&attributes), gl::STATIC_DRAW);
    };

    // Generate and bind the Vertex Array Object
    gl::GenVertexArrays(NUM_TO_MAKE, &mut vao_id);
    gl::BindVertexArray(vao_id);

    // Configure the first Vertex Attribute Poiner, vertices are structured as [x, y, z]
    gen_and_bind_vbo(&mut vbo_id_0, vertices);
    gl::VertexAttribPointer(INDEX_ATTRS[0], SIZE_ATTRS[0], TYPE_ATTR, NORMALIZED_ATTR, STRIDE_ATTRS[0], OFFSET_ATTRS[0]);  
    gl::EnableVertexAttribArray(INDEX_ATTRS[0]);

    // Configure the second Vertex Attribute Poiner, normals are structured as [x, y, z]
    gen_and_bind_vbo(&mut vbo_id_1, normals);
    gl::VertexAttribPointer(INDEX_ATTRS[1], SIZE_ATTRS[1], TYPE_ATTR, NORMALIZED_ATTR, STRIDE_ATTRS[1], OFFSET_ATTRS[1]);  
    gl::EnableVertexAttribArray(INDEX_ATTRS[1]);

    // Configure the third Vertex Attribute Poiner, colors are structured as [r, g, b, a]
    gen_and_bind_vbo(&mut vbo_id_2, colors);
    gl::VertexAttribPointer(INDEX_ATTRS[2], SIZE_ATTRS[2], TYPE_ATTR, NORMALIZED_ATTR, STRIDE_ATTRS[2], OFFSET_ATTRS[2]);  
    gl::EnableVertexAttribArray(INDEX_ATTRS[2]);

    // Generate and bind the Index Buffer Object
    gl::GenBuffers(NUM_TO_MAKE, &mut ibo_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_id);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_size_of_array(&indices), pointer_to_array(&indices), gl::STATIC_DRAW);

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

    let windowed_context    = cb.build_windowed(wb, &el).unwrap();
    let arc_pressed_keys    = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    let pressed_keys        = Arc::clone(&arc_pressed_keys);
    let arc_mouse_delta     = Arc::new(Mutex::new((0f32, 0f32)));
    let mouse_delta         = Arc::clone(&arc_mouse_delta);
    let arc_window_size     = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    let window_size         = Arc::clone(&arc_window_size);

    let render_thread = thread::spawn(move || {
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

        const TERRAIN_MODEL_PATH: &str = "./resources/lunarsurface.obj";
        const HELICOPTER_MODEL_PATH: &str = "./resources/helicopter.obj";

        let scene = scene_geometry::init_scene_geometry(TERRAIN_MODEL_PATH, HELICOPTER_MODEL_PATH); 

        // NOTE: scene_graph.rs is modified because I wanted to try Rust stuff 
        let mut scene_graph_builder = scene_graph::SceneNodeBuilder::new(); // graph root
        for i in 0..5 {
            scene_graph_builder = scene_graph_builder.add_child(scene_graph::SceneNodeBuilder::from_vao(scene.vao_ids[0], scene.triangle_counts[0])
                                                                .init(glm::zero(), 
                                                                      glm::zero(), 
                                                                      glm::vec3(1.0, 1.0, 1.0), 
                                                                      glm::vec3(0.0, 0.0, 0.0),
                                                                      "Terrain".to_string())
                                                                .add_child(scene_graph::SceneNodeBuilder::new().init_name("Heli_".to_string()+&i.to_string())
                                                                           .add_child(scene_graph::SceneNodeBuilder::from_vao(scene.vao_ids[1], scene.triangle_counts[1])
                                                                                      .init(glm::vec3(0.0, 5.0, 0.0), 
                                                                                            glm::vec3(0.2, 0.0, 0.3), 
                                                                                            glm::vec3(1.0, 1.0, 1.0), 
                                                                                            glm::vec3(0.0, 0.0, 0.0),
                                                                                            "Heli_Body".to_string())
                                                                                      .add_child(scene_graph::SceneNodeBuilder::from_vao(scene.vao_ids[2], scene.triangle_counts[2])
                                                                                                 .init(glm::zero(), 
                                                                                                       glm::vec3(0.0, 0.0, 0.5), 
                                                                                                       glm::vec3(1.0, 1.0, 1.0), 
                                                                                                       glm::vec3(0.7, 0.5, 0.0),
                                                                                                       "Heli_Door".to_string()))
                                                                                      .add_child(scene_graph::SceneNodeBuilder::from_vao(scene.vao_ids[3], scene.triangle_counts[3])
                                                                                                 .init(glm::zero(), 
                                                                                                       glm::vec3(0.0, 0.7, 0.0), 
                                                                                                       glm::vec3(1.0, 1.0, 1.0), 
                                                                                                       glm::zero(),
                                                                                                       "Heli_Main_Rotor".to_string()))
                                                                                      .add_child(scene_graph::SceneNodeBuilder::from_vao(scene.vao_ids[4], scene.triangle_counts[4])
                                                                                                 .init(glm::zero(), 
                                                                                                       glm::vec3(0.7, 0.0, 0.0), 
                                                                                                       glm::vec3(1.0, 1.0, 1.0), 
                                                                                                       glm::vec3(0.35, 2.3, 10.4),
                                                                                                       "Heli_Tail_Rotor".to_string()))
                                                                                      )));
        }

        let scene_graph_root = scene_graph_builder.build(); 
        scene_graph_root.borrow().print_tree(0);
        
        // == // Set up your shaders here
        let shadow_map_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/shadow_map.frag")
                .attach_file("./shaders/shadow_map.vert")
                .link()
        };

        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.frag")
                .attach_file("./shaders/simple.vert")
                .link()
        };


        let fancy_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/fancy.frag")
                .attach_file("./shaders/simple.vert")
                .link()
        };
        
        unsafe { shadow_map_shader.activate(); }
        let depth_mvp_location = unsafe { shadow_map_shader.get_uniform_location("depth_mvp") };

        unsafe { simple_shader.activate(); }
        let simple_shader_uniform_locations = unsafe {
            (
                simple_shader.get_uniform_location("time"),
                simple_shader.get_uniform_location("resolution"),
                simple_shader.get_uniform_location("homography"),
                simple_shader.get_uniform_location("transformation"),
            )
        };

        unsafe { fancy_shader.activate(); }
        let fancy_shader_uniform_locations = unsafe {
            (
                fancy_shader.get_uniform_location("time"),
                fancy_shader.get_uniform_location("resolution"),
                fancy_shader.get_uniform_location("homography"),
                fancy_shader.get_uniform_location("transformation"),
            )
        };

        // Storage for model transformations
        let mut node_transforms: &HashMap<String, glm::Mat4> = &mut HashMap::new();

        // Initialize shadow map variables
        let (frame_buffer_name, depth_texture) = shader::create_depth_framebuffer().unwrap();
        let depth_mvp_ptr = shader::compute_depth_mvp_matrix().as_ptr();

        // Initialize variables for camera control
        let fovy: f32 = 1.22;
        let near: f32 = 0.1;
        let far: f32 = 10000.0;
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
            const SPEED: f32 = 30.0;
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

            let initial_translation: glm::Mat4 = glm::Mat4::from([
                                                         [1.0, 0.0, 0.0, 0.0],
                                                         [0.0, 1.0, 0.0, 0.0],
                                                         [0.0, 0.0, 1.0, 25.5],
                                                         [0.0, 0.0, 0.0, 1.0]]).transpose();

            // == // Please compute camera transforms here (exercise 2 & 3)
            let homography: glm::Mat4 = perspective * angle_axis_rotation * major_axis_translation * initial_translation;

            // Rendering
            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // == // Issue the necessary gl:: commands to draw your scene here

                let set_uniforms = |view_proj_mat: &glm::Mat4, transformation_so_far: &glm::Mat4, elapsed: f32, program_id: u32| {
                    if program_id == shadow_map_shader.program_id { 
                        gl::UniformMatrix4fv(depth_mvp_location, 1, gl::FALSE, depth_mvp_ptr);
                    }
                    else if program_id == simple_shader.program_id {
                        gl::Uniform1f(simple_shader_uniform_locations.0, elapsed);
                        gl::Uniform2f(simple_shader_uniform_locations.1, width as f32, height as f32);
                        gl::UniformMatrix4fv(simple_shader_uniform_locations.2, 1, gl::FALSE, view_proj_mat.as_ptr());
                        gl::UniformMatrix4fv(simple_shader_uniform_locations.3, 1, gl::FALSE, transformation_so_far.as_ptr());
                    }
                    else if program_id == fancy_shader.program_id {
                        gl::Uniform1f(fancy_shader_uniform_locations.0, elapsed);
                        gl::Uniform2f(fancy_shader_uniform_locations.1, width as f32, height as f32);
                        gl::UniformMatrix4fv(fancy_shader_uniform_locations.2, 1, gl::FALSE, view_proj_mat.as_ptr());
                        gl::UniformMatrix4fv(fancy_shader_uniform_locations.3, 1, gl::FALSE, transformation_so_far.as_ptr());
                    }
                    else { println!("Unexpected shader program_id!"); }
                };

                scene_graph::compute_transforms(&scene_graph_root, &glm::identity(), &mut node_transforms, elapsed);
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
            
            // Default case
            _ => { }
        }
    });
}
