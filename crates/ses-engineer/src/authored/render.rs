use super::Authored;

/// Canonical rendering (not guaranteed byte-identical to input).
pub fn render(ast: &Authored) -> String {
    match ast {
        Authored::Quantity { value, unit } => {
            let mut s = value.to_string();
            if let Some(u) = unit {
                s.push(' ');
                s.push_str(&u.0);
            }
            s
        }
        Authored::FeetInches { feet, inches } => {
            let mut s = format!("{feet}'");
            if let Some(inches) = inches {
                s.push('-');
                s.push_str(&inches.to_string());
                s.push('"');
            }
            s
        }
        Authored::Product(items) => items.iter().map(render).collect::<Vec<_>>().join(" × "),
    }
}
