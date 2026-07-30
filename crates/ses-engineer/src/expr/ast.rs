use serde::{Deserialize, Serialize};

use crate::Dim;
use crate::Rational;

/// Fact reference from the pipeline facts registry, e.g. `dem.Vu`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactRef(pub String);

/// Derived quantity reference, e.g. `derived.Acv`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DerivedRef(pub String);

/// Parameter reference, e.g. `param.alpha_c`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParamRef(pub String);

/// Catalogued engine operation name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpName(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CmpOp {
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
}

/// Total, non-Turing-complete expression AST (ses-provision-dsl §3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    Quantity {
        value: Rational,
        unit: Option<String>,
    },
    Fact(FactRef),
    Derived(DerivedRef),
    Param(ParamRef),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Call {
        op: OpName,
        args: Vec<Expr>,
    },
    Min(Vec<Expr>),
    Max(Vec<Expr>),
    Piecewise {
        on: Box<Expr>,
        arms: Vec<(CmpOp, Rational, Expr)>,
        otherwise: Option<Box<Expr>>,
    },
    Table {
        key: Box<Expr>,
        key2: Option<Box<Expr>>,
        rows: Vec<(Rational, Rational, Expr)>,
    },
}

/// Boolean expression restricted from the same language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Predicate {
    Compare {
        op: CmpOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
    Not(Box<Predicate>),
    InFact {
        fact: FactRef,
        values: Vec<String>,
    },
}

/// Optional dimension metadata attached during provision compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExprDimension(pub Option<Dim>);
