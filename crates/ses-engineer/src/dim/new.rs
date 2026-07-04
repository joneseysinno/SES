use super::Dim;

impl Dim {
    /// Construct a dimension from exponent components.
    pub const fn new(force: i8, length: i8, time: i8, temp: i8) -> Self {
        Self {
            force,
            length,
            time,
            temp,
        }
    }
}
