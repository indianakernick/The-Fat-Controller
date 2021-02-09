pub trait FallibleContext {
    type Error: std::error::Error;
}
