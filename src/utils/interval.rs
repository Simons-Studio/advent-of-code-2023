use std::{
    cmp::{max, min},
    fmt::Display,
};

struct Interval<T: Ord + Eq + Display + Copy> {
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

    fn disjunct(&self, other: &Interval<T>) {}
}
