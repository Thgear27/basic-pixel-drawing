use crate::mesh::Mesh;
use russimp::node::Node;
use russimp::scene::{PostProcess, Scene};

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub name: String,
}

impl Model {
    pub fn load_from_file(path: &str) -> Model {
        let scene = Scene::from_file(path, vec![PostProcess::Triangulate]).unwrap();
        let root_node = &scene.root;

        match root_node {
            Some(node) => {
                let mut model = Model {
                    meshes: vec![],
                    name: node.name.clone(),
                };
                model.process_node(&node, &scene);

                model
            }
            None => {
                panic!("No root node found")
            }
        }
    }

    pub fn process_node(&mut self, node: &Node, scene: &Scene) {
        // Process all the node's meshes
        for mesh_index in &node.meshes {
            let russ_mesh = &scene.meshes[*mesh_index as usize];
            let mesh = Mesh::load_mesh(russ_mesh);
            self.meshes.push(mesh);
        }

        // Then do the same for each of its children
        for child in node.children.borrow_mut().iter() {
            self.process_node(child, scene);
        }
    }
}
