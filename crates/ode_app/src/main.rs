use ode_backend::Backend;
use ode_core::compositor::Compositor;
use ode_smithay::SmithayBackend;

fn main() {
    let compositor = Compositor::new();
    let backend = SmithayBackend::new();

    backend.run(compositor).unwrap();
}
