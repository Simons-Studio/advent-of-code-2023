use std::{
    cmp::{max, min},
    fmt::Display,
};

use crate::utils::incrementable::Incrementable;

pub struct Interval<T: Ord + Eq + Display + Copy + Incrementable> {
    start: T,
    end: T,
}
impl<T: Ord + Eq + Display + Copy + Incrementable> Interval<T> {
    pub fn new(start: T, end: T) -> Interval<T> {
        if start > end {
            let mut start = start;
            let mut end = end;
            Interval {
                start: end.post_inc(),
                end: start.post_inc(),
            }
        } else {
            Interval { start, end }
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
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            let overlap = Some(Interval {
                start: overlap_start,
                end: overlap_end,
            });

            let left = if other.start >= overlap_start {
                Some(Interval::new(other.start, overlap_start))
            } else {
                None
            };
            let right = if overlap_end >= other.end {
                Some(Interval::new(overlap_end, other.end))
            } else {
                None
            };

            IntervalOverlap {
                left,
                overlap,
                right,
            }
        } else {
            IntervalOverlap {
                left: None,
                overlap: Some(Interval::new(self.start, self.end)),
                right: None,
            }
        }
    }
}

pub struct IntervalOverlap<T: Ord + Eq + Display + Copy + Incrementable> {
    left: Option<Interval<T>>,
    overlap: Option<Interval<T>>,
    right: Option<Interval<T>>,
}
