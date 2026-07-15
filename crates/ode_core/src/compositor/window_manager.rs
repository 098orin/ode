use std::collections::HashMap;

use super::window::{Window, WindowId};

pub struct WindowManager {
    windows: HashMap<WindowId, Window>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn add(&mut self, window: Window) {
        self.windows.insert(window.id(), window);
    }

    pub fn remove(&mut self, id: WindowId) -> Option<Window> {
        self.windows.remove(&id)
    }

    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }
}
