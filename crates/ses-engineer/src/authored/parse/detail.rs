mod cursor;

use crate::repr_error::{AuthoredErrorKind, AuthoredParseError};

pub(super) fn err(offset: usize, kind: AuthoredErrorKind) -> AuthoredParseError {
    AuthoredParseError { offset, kind }
}

pub(super) fn contains_spaced_x(s: &str) -> bool {
    let bytes = s.as_bytes();
    for i in 1..bytes.len().saturating_sub(1) {
        if (bytes[i] == b'x' || bytes[i] == b'X')
            && bytes[i - 1].is_ascii_whitespace()
            && bytes[i + 1].is_ascii_whitespace()
        {
            return true;
        }
    }
    false
}
