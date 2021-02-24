/// A context that can fail.
///
/// Platforms implement this trait to expose their platform-specific error type.
/// This error type is wrapped in [`GenericError`](crate::GenericError). An
/// [`Error`](crate::Error) type alias is provided for convenience.
pub trait FallibleContext {
    type PlatformError: std::error::Error;
    // I would have liked to define the wrapper error here but...
    // type Error = GenericError<Self::PlatformError>;
    // "associated type defaults are unstable"
    // https://github.com/rust-lang/rust/issues/29661
}
