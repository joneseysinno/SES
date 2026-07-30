use ses_core::versioned::{FromPrev, Genesis, Lineage, Root, Versioned};

struct MockV1 {
    a: u8,
}

struct MockV2Broken {
    a: u8,
    b: u8,
}

impl Versioned for MockV1 {
    const VERSION: u8 = 1;
    type Supersedes = Root;
    type LineageVia = Genesis;
}

impl Versioned for MockV2Broken {
    const VERSION: u8 = 2;
    type Supersedes = MockV1;
    type LineageVia = FromPrev<MockV1>;
}

// Missing `impl From<MockV1> for MockV2Broken` — must not implement Lineage.

fn require_lineage<T: Lineage>() {}

fn main() {
    require_lineage::<MockV2Broken>();
}
