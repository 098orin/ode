use ode_backend::Backend;
use ode_core::compositor::Compositor;

pub struct SmithayBackend;

impl SmithayBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Backend for SmithayBackend {
    type Error = Box<dyn std::error::Error>;

    fn run(self, compositor: Compositor) -> Result<(), Self::Error> {
        println!("Smithay backend started.");

        Ok(())
    }
}
