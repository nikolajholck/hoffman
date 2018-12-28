use super::IntType;

#[derive(Clone, Debug, PartialEq)]
pub struct Interval {
    pub begin: IntType,
    pub end: IntType
}

impl Interval {
    pub fn intersects(&self, other: &Interval) -> bool {
        if self.begin < other.begin {
            other.begin < self.end
        } else {
            self.begin < other.end
        }
    }

    pub fn width(&self) -> IntType {
        self.end - self.begin
    }

    pub fn is_degenerate(&self) -> bool {
        return self.begin >= self.end
    }

    pub fn is_zero(&self) -> bool {
        self.begin == 0 && self.end == 0
    }
}
