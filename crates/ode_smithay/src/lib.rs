use ode_backend::Backend;

pub struct SmithayBackend;

impl SmithayBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Backend for SmithayBackend {
    type Error = Box<dyn std::error::Error>;

    fn run(self) -> Result<(), Self::Error> {
        println!("Smithay backend started.");

        Ok(())
    }
}
