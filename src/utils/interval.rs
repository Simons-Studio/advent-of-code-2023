use core::fmt;
use std::{
    cmp::{max, min, Ordering},
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
        other.start < self.end && self.start < other.end
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
            if self.start < overlap_start {
                let left = Interval::new(self.start, overlap_start);
                disjunction.push(left);
            }
            // Right disjunction
            if overlap_end < self.end {
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

impl<T: Ord + Eq + Display + Copy + Incrementable> Ord for Interval<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl<T: Ord + Eq + Display + Copy + Incrementable> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Eq + Display + Copy + Incrementable> fmt::Display for Interval<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}

#[cfg(test)]
mod test {
    use super::Interval;

    #[test]
    fn test_ordering() {
        let interval_1 = Interval::new(0, 10);
        let interval_2 = Interval::new(5, 15);
        assert!(interval_1 < interval_2);
    }

    #[test]
    fn test_transformation() {
        let mut interval_1 = Interval::new(0, 10);
        let interval_2 = Interval::new(5, 15);

        interval_1.transform(5);

        assert_eq!(interval_1, interval_2);
    }

    #[test]
    fn test_intersection() {
        let interval_1 = Interval::new(0, 10);
        let interval_2 = Interval::new(5, 15);
        let interval_3 = Interval::new(5, 10);
        assert_eq!(interval_1.intersection(&interval_2), Some(interval_3));

        let interval_1 = Interval::new(0, 5);
        let interval_2 = Interval::new(5, 15);
        assert_eq!(interval_1.intersection(&interval_2), None);
    }

    #[test]
    fn test_disjunction() {
        let interval_1 = Interval::new(0, 10);
        let interval_2 = Interval::new(5, 15);
        let interval_3 = Interval::new(0, 5);
        assert_eq!(interval_1.disjunction(&interval_2), vec![interval_3]);

        let interval_1 = Interval::new(0, 15);
        let interval_2 = Interval::new(5, 10);
        let interval_3 = Interval::new(0, 5);
        let interval_4 = Interval::new(10, 15);
        assert_eq!(
            interval_1.disjunction(&interval_2),
            vec![interval_3, interval_4]
        );
    }
}
