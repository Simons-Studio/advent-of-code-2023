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

    fn get_overlap(&self, other: &Interval<T>) -> IntervalOverlap<T> {
        // let mut return_val = Vec::new();
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            let overlap = Some(Interval {
                start: overlap_start,
                end: overlap_end,
            });
            let left = Interval::new(self.start, overlap_start).ok();
            let right = Interval::new(overlap_end, self.end).ok();
            IntervalOverlap {
                left,
                overlap,
                right,
            }
        } else {
            IntervalOverlap {
                left: None,
                overlap: Some(Interval::new(self.start, self.end).unwrap()),
                right: None,
            }
        }
    }

    fn disjunct(&self, other: &Interval<T>) {}
}

pub struct IntervalOverlap<T: Ord + Eq + Display + Copy> {
    left: Option<Interval<T>>,
    overlap: Option<Interval<T>>,
    right: Option<Interval<T>>,
}
