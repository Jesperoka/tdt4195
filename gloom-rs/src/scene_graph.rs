extern crate nalgebra_glm as glm;

use std::rc::Rc;
use std::cell::RefCell;

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

// fn draw_scene(node: &Node, view_projection_matrix: &glm::Mat4, transformation_so_far: &glm::Mat4) {
//     let node_borrowed = node.borrow();
//     // Your logic for rendering goes here...

//     for child in &node_borrowed.children {
//         draw_scene(child, view_projection_matrix, transformation_so_far);
//     }
// }

