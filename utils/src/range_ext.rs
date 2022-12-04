use std::ops::{Range, RangeInclusive};

pub trait RangeExt {
    /**
     * Left hand side fully contains right hand side
     *
     * Example:
     * ```
     * use aoc04::FullyContains;
     *
     * let lhs = 1..=6;
     * assert_eq!(true, lhs.fully_contains(2..=4));
     * assert_eq!(false, lhs.fully_contains(4..=8));
     * assert_eq!(false, lhs.fully_contains(8..=9));
     * ```
     */
    fn fully_contains(&self, item: &Self) -> bool;

    /**
     * Left hand side overlaps right hand side with minimal one item
     *
     * Example:
     * ```
     * use aoc04::Overlaps;
     *
     * let lhs = 1..=6;
     * assert_eq!(true, lhs.overlaps(2..=4));
     * assert_eq!(true, lhs.overlaps(4..=8));
     * assert_eq!(false, lhs.overlaps(8..=9));
     * ```
     */
    fn overlaps(&self, item: &Self) -> bool;
}

impl<Idx: PartialOrd<Idx>> RangeExt for Range<Idx> {
    #[inline]
    fn fully_contains(&self, item: &Self) -> bool {
        self.contains(&item.start) && self.contains(&item.end)
    }

    #[inline]
    fn overlaps(&self, item: &Self) -> bool {
        self.contains(&item.start) || self.contains(&item.end)
    }
}

impl<Idx: PartialOrd<Idx>> RangeExt for RangeInclusive<Idx> {
    #[inline]
    fn fully_contains(&self, item: &Self) -> bool {
        self.contains(item.start()) && self.contains(item.end())
    }

    #[inline]
    fn overlaps(&self, item: &Self) -> bool {
        self.contains(item.start()) || self.contains(item.end())
    }
}

#[test]
fn test_fully_contains() {
    let lhs = 1..6;
    assert_eq!(true, lhs.fully_contains(&(2..4)));
    assert_eq!(false, lhs.fully_contains(&(4..8)));
    assert_eq!(false, lhs.fully_contains(&(8..9)));

    let lhs = 1..=6;
    assert_eq!(true, lhs.fully_contains(&(2..=4)));
    assert_eq!(false, lhs.fully_contains(&(4..=8)));
    assert_eq!(false, lhs.fully_contains(&(8..=9)));
}

#[test]
fn test_overlaps() {
    let lhs = 1..6;
    assert_eq!(true, lhs.overlaps(&(2..4)));
    assert_eq!(true, lhs.overlaps(&(4..8)));
    assert_eq!(false, lhs.overlaps(&(8..9)));

    let lhs = 1..=6;
    assert_eq!(true, lhs.overlaps(&(2..=4)));
    assert_eq!(true, lhs.overlaps(&(4..=8)));
    assert_eq!(false, lhs.overlaps(&(8..=9)));
}
