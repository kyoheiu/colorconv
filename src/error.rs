#[derive(Debug)]
pub struct ColorConvError(pub String);

impl From<std::num::ParseIntError> for ColorConvError {
    fn from(err: std::num::ParseIntError) -> Self {
        ColorConvError(err.to_string())
    }
}
