use std::{
    cmp::{max, min},
    fmt::Display,
};

pub struct Interval<T: Ord + Eq + Display + Copy> {
    start: T,
    end: T,
}
impl<T: Ord + Eq + Display + Copy> Interval<T> {
    fn new(start: T, end: T) -> Result<Interval<T>, String> {
        if start > end {
            Err(format!("Start ({}) is not less than end ({})", start, end))
        } else {
            Ok(Interval { start, end })
        }
    }

    fn collide(&self, other: &Interval<T>) -> bool {
        other.start <= self.end && self.start <= other.end
    }

    fn intesect(&self, other: &Interval<T>) -> Option<Interval<T>> {
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            Some(Interval {
                start: overlap_start,
                end: overlap_end,
            })
        } else {
            None
        }
    }

    fn partition(&self, other: &Interval<T>) -> Vec<Interval<T>> {
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            let overlap = Interval {
                start: overlap_start,
                end: overlap_end,
            };
            let left = Interval::new(self.start, overlap_start);
            let right = Interval::new(overlap_end, self.end);
            vec![overlap]
        } else {
            vec![Interval::new(self.start, self.end).unwrap()]
        }
    }

    fn disjunct(&self, other: &Interval<T>) {}
}
