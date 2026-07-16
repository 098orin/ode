pub trait Backend {
    type Error;

    fn run(self) -> Result<(), Self::Error>;
}
