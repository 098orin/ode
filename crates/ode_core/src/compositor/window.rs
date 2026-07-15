#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub(crate) u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Normal,
    Maximized,
    Minimized,
    Fullscreen,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl WindowGeometry {
    pub const DEFAULT_WIDTH: u32 = 800;
    pub const DEFAULT_HEIGHT: u32 = 600;

    pub fn centered(width: u32, height: u32, screen_width: u32, screen_height: u32) -> Self {
        Self {
            x: ((screen_width - width) / 2) as i32,
            y: ((screen_height - height) / 2) as i32,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RestoreState {
    mode: WindowMode,
    geometry: WindowGeometry,
}

#[derive(Debug)]
pub struct Window {
    id: WindowId,
    title: String,
    app_id: String,
}

impl Window {
    pub(crate) fn new(id: WindowId) -> Self {
        Self {
            id,
            title: String::new(),
            app_id: String::new(),
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn set_app_id(&mut self, app_id: impl Into<String>) {
        self.app_id = app_id.into();
    }
}

#[derive(Debug)]
pub struct WindowState {
    geometry: WindowGeometry,
    mode: WindowMode,
    restore: Option<RestoreState>,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            geometry: WindowGeometry {
                width: WindowGeometry::DEFAULT_WIDTH,
                height: WindowGeometry::DEFAULT_HEIGHT,
                ..Default::default()
            },
            mode: WindowMode::Normal,
            restore: None,
        }
    }
}

impl WindowState {
    pub fn geometry(&self) -> WindowGeometry {
        self.geometry
    }

    pub fn set_geometry(&mut self, geometry: WindowGeometry) {
        self.geometry = geometry;
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.geometry.width = width;
        self.geometry.height = height;
    }

    pub fn mode(&self) -> WindowMode {
        self.mode
    }

    pub fn is_minimized(&self) -> bool {
        self.mode == WindowMode::Minimized
    }

    pub fn is_maximized(&self) -> bool {
        self.mode == WindowMode::Maximized
    }

    pub fn restore(&mut self) {
        if let Some(state) = self.restore.take() {
            self.mode = state.mode;
            self.geometry = state.geometry;
        } else {
            self.mode = WindowMode::Normal;
        }
    }

    pub fn set_mode(&mut self, new_mode: WindowMode) {
        if self.mode == new_mode {
            return;
        }
        if self.restore.is_none() {
            self.restore = Some(RestoreState {
                mode: self.mode,
                geometry: self.geometry,
            });
        }
        self.mode = new_mode;
    }
}
