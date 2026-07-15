use ode_core::compositor::Compositor;

pub trait Backend {
    type Error;

    fn run(self, compositor: Compositor) -> Result<(), Self::Error>;
}
