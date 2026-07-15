use super::window::WindowId;

pub struct Scene {
    nodes: Vec<SceneNode>,
}

impl Scene {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

pub enum SceneNode {
    Window(WindowId),
}
