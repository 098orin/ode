use ode_backend::Backend;
use ode_core::compositor::Compositor;
use ode_smithay::SmithayBackend;

fn main() {
    let compositor = Compositor::new();
    let backend = SmithayBackend::new(compositor);

    backend.run().unwrap();
}
