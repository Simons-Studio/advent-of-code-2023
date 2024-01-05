use std::{
    cmp::{max, min},
    fmt::Display,
};

use crate::utils::incrementable::Incrementable;

#[derive(Debug, PartialEq, Eq)]
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

    pub fn contains(&self, point: &T) -> bool {
        self.start <= *point && self.end >= *point
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

    pub fn get_overlap(&self, other: &Interval<T>) -> IntervalOverlap<T> {
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            let overlap = Some(Interval {
                start: overlap_start,
                end: overlap_end,
            });

            let mut excess = Vec::new();

            if other.start >= overlap_start {
                excess.push(Interval::new(other.start, overlap_start));
            }
            if overlap_end >= other.end {
                excess.push(Interval::new(overlap_end, other.end));
            }

            IntervalOverlap { overlap, excess }
        } else {
            IntervalOverlap {
                overlap: None,
                excess: vec![Interval::new(self.start, self.end)],
            }
        }
    }

    pub fn transform(&mut self, increment: T) {
        self.start.post_inc_by(increment);
        self.end.post_inc_by(increment);
    }
}

pub struct IntervalOverlap<T: Ord + Eq + Display + Copy + Incrementable> {
    pub overlap: Option<Interval<T>>,
    pub excess: Vec<Interval<T>>,
}
