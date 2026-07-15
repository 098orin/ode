use ode_backend::Backend;
use ode_smithay::SmithayBackend;

fn main() {
    let backend = SmithayBackend::new();

    backend.run().unwrap();
}
