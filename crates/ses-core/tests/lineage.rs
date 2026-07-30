//! Three-generation lineage chain with `From` decoders (Vocabulary §1.2).

use ses_core::versioned::{FromPrev, Genesis, Lineage, Root, Versioned, lineage_is_contiguous};

struct MockV1 {
    a: u8,
}

struct MockV2 {
    a: u8,
    b: u8,
}

struct MockV3 {
    a: u8,
    b: u8,
    c: u8,
}

impl Versioned for MockV1 {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Versioned for MockV2 {
    const VERSION: u8 = 2;
    type Supersedes = MockV1;
    type LineageVia = FromPrev<MockV1>;
}

impl Versioned for MockV3 {
    const VERSION: u8 = 3;
    type Supersedes = MockV2;
    type LineageVia = FromPrev<MockV2>;
}

impl From<MockV1> for MockV2 {
    fn from(v1: MockV1) -> Self {
        Self { a: v1.a, b: 0 }
    }
}

impl From<MockV2> for MockV3 {
    fn from(v2: MockV2) -> Self {
        Self {
            a: v2.a,
            b: v2.b,
            c: 0,
        }
    }
}

fn assert_lineage<T: Lineage>() {}

#[test]
fn three_generation_lineage_chain() {
    assert_lineage::<MockV1>();
    assert_lineage::<MockV2>();
    assert_lineage::<MockV3>();

    let v1 = MockV1 { a: 1 };
    let v2: MockV2 = v1.into();
    assert_eq!(v2.a, 1);
    assert_eq!(v2.b, 0);

    let v3: MockV3 = MockV2 { a: 2, b: 3 }.into();
    assert_eq!(v3.a, 2);
    assert_eq!(v3.b, 3);
    assert_eq!(v3.c, 0);

    assert!(lineage_is_contiguous(&[1, 2, 3]));
    assert!(!lineage_is_contiguous(&[1, 3]));
}

#[test]
fn genesis_supersedes_root() {
    fn check<T: Versioned<Supersedes = Root, LineageVia = Genesis>>() {}
    check::<MockV1>();
}
