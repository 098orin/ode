use super::window::WindowId;

pub struct Scene {
    nodes: Vec<SceneNode>,
}

impl Scene {
    pub(crate) fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add(&mut self, node: SceneNode) {
        self.nodes.push(node);
    }

    pub fn remove(&mut self, node: &SceneNode) {
        self.nodes.retain(|n| !n.eq(node));
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn nodes(&self) -> &[SceneNode] {
        &self.nodes
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneNode {
    Window(WindowId),
    Cursor,
}
