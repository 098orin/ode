use std::collections::HashMap;

use super::window::{Window, WindowId, WindowState};

pub struct WindowManager {
    windows: HashMap<WindowId, Window>,
    states: HashMap<WindowId, WindowState>,
    focused: Option<WindowId>,
    next_id: u64,
}

impl WindowManager {
    pub(crate) fn new() -> Self {
        Self {
            windows: HashMap::new(),
            states: HashMap::new(),
            focused: None,
            next_id: 0,
        }
    }

    pub(crate) fn new_window(&mut self) -> WindowId {
        let id = WindowId(self.next_id);
        self.next_id += 1;
        self.windows.insert(id, Window::new(id));
        self.states.insert(id, WindowState::default());
        id
    }

    pub fn remove(&mut self, id: WindowId) {
        if self.focused == Some(id) {
            self.focused = None;
        }
        self.windows.remove(&id);
        self.states.remove(&id);
    }

    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn get_state(&self, id: WindowId) -> Option<&WindowState> {
        self.states.get(&id)
    }

    pub fn get_state_mut(&mut self, id: WindowId) -> Option<&mut WindowState> {
        self.states.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Window> {
        self.windows.values()
    }

    pub fn len(&self) -> usize {
        self.windows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    pub fn focus(&mut self, id: WindowId) {
        self.focused = Some(id);
    }

    pub fn unfocus(&mut self) {
        self.focused = None;
    }

    pub fn focused(&self) -> Option<WindowId> {
        self.focused
    }
}
