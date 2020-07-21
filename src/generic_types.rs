use std::cmp::Ordering;
use std::ops::{
    Div,
    Mul,
    Sub,
    Add
};

pub type Num = f64;
pub type Int = i64;
pub type Input = [Num];
pub type IntInput = [Int];


trait EulerNumber {
    fn e(&self) -> Self;
}

impl EulerNumber for f64 {
    #[inline]
    fn e(&self) -> Self {
        std::f64::consts::E
    }
}

impl EulerNumber for f32 {
    #[inline]
    fn e(&self) -> Self {
        std::f32::consts::E
    }
}

pub const MAX_DIGITS: u64 = 18_446_744_073_709_551_615u64 - 8_446_744_073_709_551_615u64;
// fraction must not exceed 9999999999999999999(20 digits at maximum)
pub const LARGEST_FRACTION: u64 = 18_446_744_073_709_551_615u64 - 8_446_744_073_709_551_615u64 - 1;
pub const FRACTION_EQUIVALENT_TO_ONE: u64 = 18_446_744_073_709_551_615u64 - 8_446_744_073_709_551_615u64 - 1;
pub const ZERO_VALUE: NotFloat = NotFloat {fract: 0, trunc: 0};

/// this type was introduced to workaround with floating point number's lack of Eq and Ordering.
#[derive(Clone, Copy, Hash, Eq, Default)]
pub struct NotFloat {
    fract: u64,
    trunc: i64,
}

impl std::fmt::Debug for NotFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl NotFloat {
    pub fn has_fraction(&self) -> bool {
        self.fract != 0
    }
    pub fn is_zero(&self) -> bool {
        (0, 0) == (self.fract, self.trunc)
    }
    pub fn frac_digits(&self) -> usize {
        let ten = 10u64;
        if self.fract < 10 {
            return 1
        }
        for r in (2..20u32).rev() {
            if self.fract < ten.pow(r) {
                return r as usize
            }
        }
        20
    }

    /// constructor function.
    /// if value of fract exceeds the value specified with LARGEST_FRACTION
    /// the value will be modified according to correct_fraction function
    pub fn new(trunc: i64, fract: u64) -> Self {
        let mut v = Self {
            trunc,
            fract
        };
        v.correct_fraction();
        v
    }

    /// if the value of fraction exceeds value specified with LARGEST_FRACTION,
    /// fract will be subtracted with LARGEST_FRACTION and trunc will be increased by 1
    pub fn correct_fraction(&mut self) -> bool {
        let Self {trunc, fract} = self;
        let check = *fract > LARGEST_FRACTION;
        if check {
            *trunc += 1;
            *fract -= LARGEST_FRACTION;
        }

        check
    }

    /// multiplies a value with a number that is smaller than 0 < n < 1
    fn multiply_fract(&self, n: u64) -> Self {
        let mut new = Self::new(0, 0);
        // let t = 2783 * 0.1 = 27830. 
        // Note that 0.1 in this case will 1000000000000000000
        let t = self.trunc * (n as i64);
        let ten = 10i64;

        for i in 0..21 {
            let powed = ten.pow(i);
            if powed <= t && t < powed {
                // divide t with powered 10
                // let t = 2783 / 1000000000000000000 = 0
                new.trunc = t / powed;
                // subtract t with powered new_trunc
                new.fract = (t - (new.trunc * powed)) as u64;
                break
            }
            if i == 20 {
                panic!("overflow")
            }
        };

        new
    }

    /// returns a fractional part of a number
    pub fn fract(&self) -> u64 {
        self.fract
    }
    /// returns a part of a number that is not fractional
    pub fn trunc(&self) -> i64 {
        self.trunc
    }
    // euler number
    pub const E: Self = Self {
        trunc: 2,
        fract: 7182818284590452353
    };
}

impl ToString for NotFloat {
    fn to_string(&self) -> String {
        format!("{}.{}", self.trunc, self.fract)
    }
}

impl EulerNumber for NotFloat {
    fn e(&self) -> Self {
        Self::E
    }
}

impl From<f64> for NotFloat {
    fn from(x: f64) -> Self {
        NotFloat{
            trunc: x.trunc() as i64,
            fract: (x.fract() * MAX_DIGITS as Num) as u64
        }
    }
}

impl From<NotFloat> for f64 {
    fn from(x: NotFloat) -> Self {
        x.to_string().parse().unwrap()
    }
}

impl From<NotFloat> for i64 {
    fn from(x: NotFloat) -> Self {
        x.trunc
    }
}

impl PartialEq for NotFloat {
    fn eq(&self, other: &Self) -> bool {
        self.fract == other.fract 
        && self.trunc == other.trunc
    }
}

impl PartialOrd for NotFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.trunc.partial_cmp(&other.trunc) {
            Some(Ordering::Equal) => {
                self.fract.partial_cmp(&other.fract)
            }
            other => other
        }
    }
}

impl Ord for NotFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.trunc > other.trunc {
            Ordering::Less
        } else if self.trunc < other.trunc {
            Ordering::Greater
        } else { // assume its equal
            if self.fract > other.fract {
                Ordering::Less
            } else if self.fract < other.fract {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl Add for NotFloat {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.is_zero() {
            other
        } else if other.is_zero() {
            self
        } else {
            let (fract, overflow) = self.fract.overflowing_add(other.fract);
            let mut trunc = self.trunc + other.trunc;
            if overflow {
                trunc += 1
            };
    
            Self {
                trunc, fract
            }
        }
    }
}

impl Sub for NotFloat {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        let mut trunc = self.trunc - other.trunc;
        let (fract, overflow) = self.fract.overflowing_sub(other.fract);
        if overflow {
            trunc -= 1;
        }

        Self {
            fract, trunc
        }
    }
}

// todo
impl Mul for NotFloat {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        // optimizers
        let _optimized = loop {
            let value = if self == ZERO_VALUE || other == ZERO_VALUE {
                ZERO_VALUE
            } else if other.trunc == 1 && other.fract == 0 {
                self
            } else if self.trunc == 1 && self.fract == 0 {
                other
            } else {
                break
            };
            return value
        };

        // evaluate against trunc value
        let evaluate_trunc = {
            let (range, against) = if other.trunc > self.trunc {
                (0..self.trunc, other)
            } else {
                (0..other.trunc, self)
            };

            range.fold(ZERO_VALUE, |acc, _i| acc + against)
        };

        let evaluate_fract = {
            unimplemented!("");
            let (range, against) = if other.fract > self.fract {
                (0..self.fract, other)
            } else {
                (0..other.fract, self)
            };

            let fract = range.fold(ZERO_VALUE, |acc, _i| acc + against);
            fract
        };

        evaluate_fract + evaluate_trunc
    }
}

// todo
impl Div for NotFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let divided = self.trunc / rhs.trunc;
        
        if self.fract == 0 && rhs.fract == 0 {
            Self {
                trunc: divided,
                fract: 0
            }
        } else if self.trunc == 0 || rhs.trunc == 0 {
            if self.trunc == 0 {
                rhs
            } else if rhs.trunc == 0 {
                self
            } else {
                unimplemented!("")
            }
        } else {
            unimplemented!("")
        }
    } 
}