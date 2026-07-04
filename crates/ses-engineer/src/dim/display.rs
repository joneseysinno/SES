use core::fmt;

use super::Dim;

impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dimensionless() {
            return f.write_str("—");
        }
        let mut first = true;
        let parts: [(&str, i8); 4] = [
            ("F", self.force),
            ("L", self.length),
            ("T", self.time),
            ("Θ", self.temp),
        ];
        for (sym, exp) in parts {
            if exp == 0 {
                continue;
            }
            if !first {
                f.write_str("·")?;
            }
            first = false;
            f.write_str(sym)?;
            write_exp(f, exp)?;
        }
        Ok(())
    }
}

fn write_exp(f: &mut fmt::Formatter<'_>, exp: i8) -> fmt::Result {
    match exp {
        1 => Ok(()),
        -1 => f.write_str("⁻¹"),
        2 => f.write_str("²"),
        -2 => f.write_str("⁻²"),
        3 => f.write_str("³"),
        -3 => f.write_str("⁻³"),
        4 => f.write_str("⁴"),
        -4 => f.write_str("⁻⁴"),
        n if n > 0 => write!(f, "^{n}"),
        n => write!(f, "^{n}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_named_constants() {
        assert_eq!(Dim::STRESS.to_string(), "F·L⁻²");
        assert_eq!(Dim::MOMENT.to_string(), "F·L");
        assert_eq!(Dim::DIMENSIONLESS.to_string(), "—");
    }
}
