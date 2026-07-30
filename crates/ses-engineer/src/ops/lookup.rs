use super::EmpiricalOp;

/// Look up an empirical operation by name (Vocabulary §1.3).
pub fn lookup(name: &str) -> Option<&'static EmpiricalOp> {
    super::catalog::catalog().iter().find(|op| op.name == name)
}
