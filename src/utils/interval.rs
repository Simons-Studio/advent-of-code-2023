use std::{
    cmp::{max, min},
    fmt::Display,
};

use crate::utils::incrementable::Incrementable;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    pub fn collide(&self, other: &Interval<T>) -> bool {
        other.start <= self.end && self.start <= other.end
    }

    pub fn intersection(&self, other: &Interval<T>) -> Option<Interval<T>> {
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            let overlap = Interval {
                start: overlap_start,
                end: overlap_end,
            };
            Some(overlap)
        } else {
            None
        }
    }

    pub fn disjunction(&self, other: &Interval<T>) -> Vec<Interval<T>> {
        let mut disjunction = Vec::new();
        if self.collide(other) {
            let overlap_start = max(self.start, other.start);
            let overlap_end = min(self.end, other.end);
            // Left disjunction
            if self.start <= overlap_start {
                let left = Interval::new(self.start, overlap_start);
                disjunction.push(left);
            }
            // Right disjunction
            if overlap_end <= self.end {
                let right = Interval::new(overlap_end, self.end);
                disjunction.push(right);
            }
        }
        disjunction
    }

    pub fn transform(&mut self, increment: T) {
        self.start.post_inc_by(increment);
        self.end.post_inc_by(increment);
    }
}
