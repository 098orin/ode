#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u64);

#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum WindowMode {
    Normal,
    Maximized,
    Minimized,
    Fullscreen,
}

pub struct RestoreState {
    mode: WindowMode,
    geometry: WindowGeometry,
}

pub struct WindowState {
    mode: WindowMode,
    restore: Option<RestoreState>,
}

pub struct Window {
    id: WindowId,
    geometry: WindowGeometry,
    state: WindowState,
}

impl Window {
    pub fn new(id: WindowId) -> Self {
        Self {
            id,
            geometry: WindowGeometry {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            },
            state: WindowState {
                mode: WindowMode::Normal,
                restore: None,
            },
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn geometry(&self) -> WindowGeometry {
        self.geometry
    }
}
