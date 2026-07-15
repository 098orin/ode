mod scene;
mod window;
mod window_manager;

pub use scene::Scene;

use self::window_manager::WindowManager;

pub struct Compositor {
    windows: WindowManager,
    scene: Scene,
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
}
