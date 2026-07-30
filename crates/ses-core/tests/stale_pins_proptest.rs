//! Staleness predicate property tests (Vocabulary §1.2).

use proptest::prelude::*;
use ses_core::provenance::{Pin, Revision, any_stale_pins, pin_is_stale};

proptest! {
    #[test]
    fn pin_not_stale_at_same_revision(rev in 0_u64..10_000) {
        let r = Revision(rev);
        let pin = Pin::new(1_u64, r);
        prop_assert!(!pin_is_stale(&pin, r));
    }

    #[test]
    fn pin_stale_when_current_is_newer(
        pinned in 0_u64..10_000,
        delta in 1_u64..1_000,
    ) {
        let pin = Pin::new(42_u64, Revision(pinned));
        let current = Revision(pinned.saturating_add(delta));
        prop_assert!(pin_is_stale(&pin, current));
    }

    #[test]
    fn pin_not_stale_when_current_is_older(
        current in 0_u64..10_000,
        delta in 1_u64..1_000,
    ) {
        let pinned = current.saturating_add(delta);
        let pin = Pin::new(7_u64, Revision(pinned));
        prop_assert!(!pin_is_stale(&pin, Revision(current)));
    }

    #[test]
    fn any_stale_pins_detects_one_stale(
        stale_id in 1_u64..100,
        fresh_id in 101_u64..200,
    ) {
        let pins = [
            Pin::new(stale_id, Revision(1)),
            Pin::new(fresh_id, Revision(5)),
        ];
        let lookup = |id: &u64| {
            if *id == stale_id {
                Revision(3)
            } else {
                Revision(5)
            }
        };
        prop_assert!(any_stale_pins(&pins, lookup));
    }

    #[test]
    fn any_stale_pins_all_fresh(rev in 1_u64..10_000) {
        let pins = [Pin::new(1_u64, Revision(rev)), Pin::new(2_u64, Revision(rev))];
        let lookup = |_id: &u64| Revision(rev);
        prop_assert!(!any_stale_pins(&pins, lookup));
    }
}
