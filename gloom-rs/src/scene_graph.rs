extern crate nalgebra_glm as glm;

use std::{rc::Rc, cell::RefCell, ptr};

pub type Node = Rc<RefCell<SceneNode>>;

pub struct SceneNode {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    pub reference_point: glm::Vec3,
    pub vao_id: u32,
    pub index_count: i32,
    pub children: Vec<Node>,
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
            })),
        }
    }

    pub fn init(self, position: glm::Vec3, rotation: glm::Vec3, scale: glm::Vec3, reference_point: glm::Vec3) -> Self {
        {
        let mut node = (*self.node).borrow_mut();
        node.position = position;
        node.rotation = rotation;
        node.scale = scale;
        node.reference_point = reference_point;
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
                VAO:       {}
                Indices:   {}
                Children:  {}
                Position:  [{:.2}, {:.2}, {:.2}]
                Rotation:  [{:.2}, {:.2}, {:.2}]
                Scale:     [{:.2}, {:.2}, {:.2}]
                Reference: [{:.2}, {:.2}, {:.2}]
            }}",
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
            print!("child id  {}  ", child.borrow().vao_id);
            child.borrow().print_tree(depth + 1);
        }
    }
}

pub unsafe fn draw_scene<F>(node: &Node, view_projection_matrix: &glm::Mat4, transformation_so_far: &glm::Mat4, set_uniforms: &F) 
    where 
        F: Fn(&glm::Mat4, &glm::Mat4),
    {
    let node_borrow = node.borrow();

    // Transformations
    let mut transformation_so_far = *transformation_so_far; 

    if node_borrow.index_count > 0 {

        let (tx, ty, tz) = (node_borrow.position[0], node_borrow.position[1], node_borrow.position[2]); 
        let (rx, ry, rz) = (node_borrow.reference_point[0], node_borrow.reference_point[1], node_borrow.reference_point[2]); 
        let (gamma, beta, alpha) = (node_borrow.rotation[0], node_borrow.rotation[1], node_borrow.rotation[2]);
        
        let translation: glm::Mat4 = glm::mat4(
            1.0, 0.0, 0.0, rx,
            0.0, 1.0, 0.0, ry,
            0.0, 0.0, 1.0, rz,
            0.0, 0.0, 0.0, 1.0, 
            );

        let rotation: glm::Mat4 = glm::mat4(
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

        // print!("{}", translation);
        // print!("{}", rotation);

        transformation_so_far = rotation * transformation_so_far;                

        // Render
        set_uniforms(view_projection_matrix, &transformation_so_far);
        gl::BindVertexArray(node_borrow.vao_id); 
        gl::DrawElements(gl::TRIANGLES, node_borrow.index_count, gl::UNSIGNED_INT, ptr::null());
    }

    // Recursion
    for child in &node_borrow.children {
        draw_scene(child, view_projection_matrix, &transformation_so_far, set_uniforms);
    }
}

