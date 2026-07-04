use core::fmt;

use super::Rational;

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            return write!(f, "{}", self.num);
        }
        write!(f, "{}/{}", self.num, self.den)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_format() {
        assert_eq!(Rational::from_int(3).to_string(), "3");
        assert_eq!(Rational::new(3, 4).unwrap().to_string(), "3/4");
    }
}
