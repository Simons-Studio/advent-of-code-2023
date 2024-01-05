// Copied from aSpex in
// https://stackoverflow.com/questions/41669634/implementing-a-generic-incrementable-trait-in-rust
pub trait Incrementable: Copy + std::ops::AddAssign<Self> {
    fn one() -> Self;

    fn post_inc(&mut self) -> Self {
        self.post_inc_by(Self::one())
    }

    fn post_inc_by(&mut self, n: Self) -> Self {
        let tmp = *self;
        *self += n;
        tmp
    }
}

impl Incrementable for u8 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for u16 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for u32 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for u64 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for i8 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for i16 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for i32 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for i64 {
    fn one() -> Self {
        1
    }
}
impl Incrementable for f32 {
    fn one() -> Self {
        1.0
    }
}
impl Incrementable for f64 {
    fn one() -> Self {
        1.0
    }
}
