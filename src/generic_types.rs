use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt;

pub type Num = f64;
pub type Int = i64;
pub type Input = [Num];
pub type IntInput = [Int];

#[derive(Default, PartialOrd, PartialEq, Copy, Clone)]
pub struct OrdFloat(pub f64);
impl Eq for OrdFloat {}
impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if other.0 < self.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl From<OrdFloat> for f64 {
    fn from(o: OrdFloat) -> Self {
        o.0
    }
}
impl Hash for OrdFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_be_bytes().iter().for_each(|i| {
            state.write_u8(*i);
        })
    }
}
impl fmt::Debug for OrdFloat {
    fn fmt(&self, f:  &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.0.to_string())
    }
}
impl fmt::Display for OrdFloat {
    fn fmt(&self, f:  &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.0.to_string())
    }
}