mod scene;
mod window;
mod window_manager;

pub use scene::{Scene, SceneNode};
pub use window::{Window, WindowGeometry, WindowId, WindowMode, WindowState};

use self::window_manager::WindowManager;

pub struct Compositor {
    windows: WindowManager,
    scene: Scene,
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            windows: WindowManager::new(),
            scene: Scene::new(),
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn windows(&self) -> &WindowManager {
        &self.windows
    }

    pub fn focused_window(&self) -> Option<WindowId> {
        self.windows.focused()
    }

    pub fn create_window(&mut self) -> WindowId {
        let id = self.windows.new_window();
        self.scene.add(SceneNode::Window(id));
        id
    }

    pub fn destroy_window(&mut self, id: WindowId) {
        self.windows.remove(id);
        self.scene.remove(&SceneNode::Window(id));
    }

    pub fn focus_window(&mut self, id: WindowId) {
        self.windows.focus(id);
    }

    pub fn unfocus_all(&mut self) {
        self.windows.unfocus();
    }

    pub fn update_scene(&mut self) {
        self.scene.clear();
        for window in self.windows.iter() {
            let state = self.windows.get_state(window.id());
            if state.is_none_or(|s| !s.is_minimized()) {
                self.scene.add(SceneNode::Window(window.id()));
            }
        }
        self.scene.add(SceneNode::Cursor);
    }
}
