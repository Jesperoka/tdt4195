extern crate nalgebra_glm as glm;

use std::{rc::Rc, cell::RefCell, cell::RefMut, ptr, collections::HashMap};
use crate::toolbox;
use crate::shader::Shader;

pub type Node = Rc<RefCell<SceneNode>>;

pub struct SceneNode {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    pub reference_point: glm::Vec3,
    pub vao_id: u32,
    pub index_count: i32,
    pub children: Vec<Node>,
    pub name: String,
}

pub struct SceneNodeBuilder {
    node: Node,
}

impl SceneNodeBuilder {
    pub fn new() -> Self {
        SceneNodeBuilder {
            node: Rc::new(RefCell::new(SceneNode {
                position: glm::zero(),
                rotation: glm::zero(),
                scale: glm::vec3(1.0, 1.0, 1.0),
                reference_point: glm::zero(),
                vao_id: 0,
                index_count: -1,
                children: vec![],
                name: "".to_string(),
            })),
        }
    }

    pub fn from_vao(vao_id: u32, index_count: i32) -> Self {
        SceneNodeBuilder {
            node: Rc::new(RefCell::new(SceneNode {
                position: glm::zero(),
                rotation: glm::zero(),
                scale: glm::vec3(1.0, 1.0, 1.0),
                reference_point: glm::zero(),
                vao_id,
                index_count,
                children: vec![],
                name: "".to_string(),
            })),
        }
    }

    pub fn init(self, position: glm::Vec3, rotation: glm::Vec3, scale: glm::Vec3, reference_point: glm::Vec3, name: String) -> Self {
        {
        let mut node = (*self.node).borrow_mut();
        node.position = position;
        node.rotation = rotation;
        node.scale = scale;
        node.reference_point = reference_point;
        node.name = name;
        }
        return self; 
    }

    // Overload to just give name to unintialized nodes
    pub fn init_name(self, name: String) -> Self {
        {
        let mut node = (*self.node).borrow_mut(); 
        node.name = name; 
        } 
        return self;
    }

    pub fn add_child(self, child: SceneNodeBuilder) -> Self {
        (*self.node).borrow_mut().children.push(child.node);
        return self;
    }

    pub fn build(self) -> Node {
        return self.node;
    }

}

impl SceneNode {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!(
            "SceneNode {{
                Name:      {}
                VAO:       {}
                Indices:   {}
                Children:  {}
                Position:  [{:.2}, {:.2}, {:.2}]
                Rotation:  [{:.2}, {:.2}, {:.2}]
                Scale:     [{:.2}, {:.2}, {:.2}]
                Reference: [{:.2}, {:.2}, {:.2}]
            }}",
            self.name,
            self.vao_id,
            self.index_count,
            self.children.len(),
            self.position.x,
            self.position.y,
            self.position.z,
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
            self.scale.x,
            self.scale.y,
            self.scale.z,
            self.reference_point.x,
            self.reference_point.y,
            self.reference_point.z,
            );
    }

    pub fn print_tree(&self, depth: usize) {
        // Print the current node with indentation
        let indentation = "  ".repeat(depth);
        println!("{}Node at depth {}: ", indentation, depth);
        self.print();

        // Recursively print children
        for child in &self.children {
            child.borrow().print_tree(depth + 1);
        }
    }
}

// Recursively computes and stores transforms for each drawable node
pub unsafe fn compute_transforms(node: &Node,
                            transformation_so_far: &glm::Mat4, 
                            node_transforms: &mut HashMap<String, glm::Mat4>,
                            elapsed: f32, 
                            // set_uniforms: &F, 
                            // shadow_map_shader: &Shader, 
                            // simple_shader: &Shader,
                            // fancy_shader: &Shader
                            ) {

    let offset: f32 = 0.77*extract_heli_number(&*node.borrow_mut().name);
    time_dependent_animation_step(node.borrow_mut(), elapsed);
    let node_borrow = node.borrow();
        
    // Transformations
    let mut transformation_so_far = *transformation_so_far; 

    if node_borrow.index_count > 0 {

        let (tx, ty, tz) = (node_borrow.position[0], node_borrow.position[1], node_borrow.position[2]); 
        let (rx, ry, rz) = (node_borrow.reference_point[0], node_borrow.reference_point[1], node_borrow.reference_point[2]); 
        let (gamma, beta, alpha) = (node_borrow.rotation[0], node_borrow.rotation[1], node_borrow.rotation[2]);
        
        let ref_translation: glm::Mat4 = glm::mat4(
            1.0, 0.0, 0.0, rx,
            0.0, 1.0, 0.0, ry,
            0.0, 0.0, 1.0, rz,
            0.0, 0.0, 0.0, 1.0, 
            );

        let inv_ref_translation: glm::Mat4 = glm::mat4(
            1.0, 0.0, 0.0, -rx,
            0.0, 1.0, 0.0, -ry,
            0.0, 0.0, 1.0, -rz,
            0.0, 0.0, 0.0, 1.0, 
            );

        let roto_translation: glm::Mat4 = glm::mat4(
            f32::cos(alpha)*f32::cos(beta), 
            f32::cos(alpha)*f32::sin(beta)*f32::sin(gamma) - f32::sin(alpha)*f32::cos(gamma), 
            f32::cos(alpha)*f32::sin(beta)*f32::cos(gamma) + f32::sin(alpha)*f32::sin(gamma), 
            tx,

            f32::sin(alpha)*f32::cos(beta), 
            f32::sin(alpha)*f32::sin(beta)*f32::sin(gamma) + f32::cos(alpha)*f32::cos(gamma), 
            f32::sin(alpha)*f32::sin(beta)*f32::cos(gamma) - f32::cos(alpha)*f32::sin(gamma), 
            ty,

            -f32::sin(beta), 
            f32::cos(beta)*f32::sin(gamma), 
            f32::cos(beta)*f32::cos(gamma), 
            tz,

            0.0, 0.0, 0.0, 1.0
            );

        transformation_so_far = transformation_so_far * ref_translation * roto_translation * inv_ref_translation;                

        // // Shadow map
        // shadow_map_shader.activate();
        // /* set any uniforms if any are at all needed */

        // // Render
        // if !node_borrow.name.starts_with("Heli_") { 
        //     simple_shader.activate(); 
        //     set_uniforms(view_projection_matrix, &transformation_so_far, elapsed, simple_shader.program_id);
        // } else { 
        //     fancy_shader.activate(); 
        //     set_uniforms(view_projection_matrix, &transformation_so_far, elapsed, fancy_shader.program_id);
        // }
        // gl::BindVertexArray(node_borrow.vao_id); 
        // gl::DrawElements(gl::TRIANGLES, node_borrow.index_count, gl::UNSIGNED_INT, ptr::null());

        // Cache result
        node_transforms.insert(node_borrow.name.clone(), transformation_so_far.clone());
    }

    // Recursion
    for child in &node_borrow.children {
        compute_transforms(child, 
                   &transformation_so_far, 
                   node_transforms, 
                   elapsed+offset, 
                   );
    }
}

pub enum RenderMode {
    ShadowPass,
    Normal,
}

pub unsafe fn draw_scene<F>(
    node: &Node,
    view_projection_matrix: &glm::Mat4,
    elapsed: f32,
    set_uniforms: &F,
    shadow_map_shader: &Shader,
    simple_shader: &Shader,
    fancy_shader: &Shader,
    node_transforms: &HashMap<String, glm::Mat4>,
    render_mode: RenderMode
) where 
    F: Fn(&glm::Mat4, &glm::Mat4, f32, u32),
{
    let node_borrow = node.borrow();
    let transformation_so_far = node_transforms.get(&*node_borrow.name).expect("Transformation not found!");

    match render_mode {
        RenderMode::ShadowPass => {
            // Shadow pass
            shadow_map_shader.activate();
            // Set any uniforms if any are at all needed

            // Bind and draw
            gl::BindVertexArray(node_borrow.vao_id); 
            gl::DrawElements(gl::TRIANGLES, node_borrow.index_count, gl::UNSIGNED_INT, ptr::null());
        },
        RenderMode::Normal => {
            // Render
            if !node_borrow.name.starts_with("Heli_") { 
                simple_shader.activate(); 
                set_uniforms(view_projection_matrix, &transformation_so_far, elapsed, simple_shader.program_id);
            } else { 
                fancy_shader.activate(); 
                set_uniforms(view_projection_matrix, &transformation_so_far, elapsed, fancy_shader.program_id);
            }

            // Bind and draw
            gl::BindVertexArray(node_borrow.vao_id); 
            gl::DrawElements(gl::TRIANGLES, node_borrow.index_count, gl::UNSIGNED_INT, ptr::null());
        }
    }

    // Recursion
    for child in &node_borrow.children {
        draw_scene(child, 
                   view_projection_matrix, 
                   elapsed + offset, 
                   set_uniforms, 
                   shadow_map_shader, 
                   simple_shader, 
                   fancy_shader,
                   node_transforms,
                   render_mode);
    }
}


// Side effect: mutates passed node to perform time based animation step 
fn time_dependent_animation_step(mut node_mutable_borrow: RefMut<SceneNode>, elapsed: f32) { 
    const ROT_SPEED: f32 = 1000.0;
    match &*node_mutable_borrow.name {
        "Heli_Body" => {
            let heading = toolbox::simple_heading_animation(elapsed);
            node_mutable_borrow.position[0] = heading.x;
            node_mutable_borrow.position[2] = heading.z;
            node_mutable_borrow.rotation[0] = heading.pitch;
            node_mutable_borrow.rotation[1] = heading.yaw;
            node_mutable_borrow.rotation[2] = heading.roll; 
        }
        // "Heli_Door" => {

        // }
        "Heli_Main_Rotor" => {
            node_mutable_borrow.rotation[1] = ROT_SPEED * (elapsed % (2.0 * std::f32::consts::PI));
        }
        "Heli_Tail_Rotor" => {
            node_mutable_borrow.rotation[0] = ROT_SPEED * (elapsed % (2.0 * std::f32::consts::PI));
        }
        _ => {}
    }
}

// Defaults to 0
fn extract_heli_number(s: &str) -> f32 {
    const PREFIX: &str = "Heli_";
    
    if s.starts_with(PREFIX) {
        let num_str = &s[PREFIX.len()..];
        
        if let Ok(num) = num_str.parse::<i32>() {
            return num as f32;
        }
    }

    return 0.0;
}
