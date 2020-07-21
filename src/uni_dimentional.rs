/// functions that takes only one argument will be found here

use std::collections::{
    HashMap,
    HashSet,
    BTreeMap
};

use crate::generic_types::{
    NotFloat,
    Input,
    Num
};


pub fn sort_float(input: &Input) -> Vec<Num> {
    let mut v = Vec::with_capacity(input.len());
    input.iter().for_each(|e | v.push(NotFloat::from(*e)));
    v.sort();
    v.iter()
        .map(|e| f64::from(*e))
        .collect::<Vec<f64>>()
}

/// assumption
/// all functions assumes that given data is sorted

/// calculates the sum of the input
pub fn sum(input: &Input) -> Num {
    input.iter().fold(0 as Num, |acc, n| acc + n)
}

/// calculates the median of the input
pub fn median(input: &Input) -> Num {   
    let half = input.len() / 2;
    let median = if half % 2 == 1 {
        let mut skipper = input.iter().skip(half);
        let mut n = if let Some(e) = skipper.next() {
            *e
        } else {
            unreachable!("")
        };
        if let Some(e) = skipper.next() {
            n += *e
        };

        n / 2 as Num
    } else {
        *input.iter().skip(half).next().unwrap()
    };

    median
}

/// find the value that is most frequently seen in the input
/// first value in the tuple is the number of times that the value is observed
/// second value is the keys that are observed 
pub fn mode(input: &Input) -> (usize, HashSet<NotFloat>) {
    let mut max = 0;
    let mut set = HashSet::new();

    let mut map = HashMap::new();
    input.iter().for_each(|i| {
        let i = NotFloat::from(*i);
        map.entry(i).or_insert(0 as usize);

        let count = map.get_mut(&i).unwrap();
        *count += 1;
        
        let count_now = *count;

        if max == count_now {
            set.insert(i);
        } else if max < count_now {
            max = count_now;
            set.clear();
            set.insert(i);
        }
    });

    let item = *set.iter().next().unwrap();
    let key = *set.get(&item).unwrap();
    let freq = *map.get(&key).unwrap();

    (freq, set)
}

/// counts how many times the each value appears within the given input
pub fn frequency_distribution(input: &Input) -> BTreeMap<NotFloat, Num> {
    let mut map = BTreeMap::new();

    input.iter().for_each(|i| {
        let i = NotFloat::from(*i);
        match map.get_mut(&i) {
            Some(count) => *count += 1.,
            None => {
                map.insert(i, 1 as Num);
            }
        };
    });

    map
}

/// divides each value with length of given input.
pub fn probability_distribution(input: &Input) -> BTreeMap<NotFloat, Num> {
    let f_len = input.len() as Num;
    let mut tree = frequency_distribution(input);
    tree.iter_mut().for_each(|(_k, v)| *v /= f_len);
    tree
}

pub fn max(input: &Input) -> Num {
    input.iter().fold(0 as Num, |acc, n| {
        if &acc > n {
            acc
        } else {
            *n
        }
    })
}

pub fn min(input: &Input) -> Num {
    input.iter().fold(0 as Num, |acc, n| {
        if &acc < n {
            acc
        } else {
            *n
        }
    })
}

pub fn mean(input: &Input) -> Num {
    let sum = input.iter().fold(0 as Num, |acc, n| acc + n);
    sum / input.len() as Num
}

pub fn geometric_mean(input: &Input) -> Num {
    let num_one = 1 as Num;
    let num_zero = 0 as Num;
    let p = input.iter().fold(num_zero, |acc, n| {
        if acc == num_zero {
            *n
        } else {
            acc * *n
        }
    });

    let thing = num_one / input.len() as Num;
    p.powf(thing)
}

pub fn harmonic_mean(input: &Input) -> Num {
    let p = input
        .iter()
        .fold(0 as Num, |acc, n| acc + ( 1 as Num / n));

    input.len() as Num / p
}

pub fn population_variance(input: &Input) -> Num {
    let avg = mean(input);
    let var = input.iter().fold(0 as Num, |acc, n| {
        acc + ((n - avg).powi(2))
    });

    var / input.len() as Num
}

pub fn sample_variance(input: &Input) -> Num {
    let avg = mean(input);
    let var = input.iter().fold(0 as Num, |acc, n| {
        acc + ((n - avg).powi(2))
    });

    (var / ((input.len()-1) as Num)).sqrt()
}

pub fn cumulative_sum(input: &Input) -> Vec<Num> {
    let mut sum = 0 as Num;
    let mut vec = Vec::with_capacity(input.len());

    input.iter().for_each(|e| {
        sum += e;
        vec.push(sum);
    });

    vec
}

pub fn standard_deviation_population(input: &Input) -> Num {
    population_variance(input).powf(0.5)
}

pub fn standard_deviation_sample(input: &Input) -> Num { 
    sample_variance(input).powf(0.5)
}

pub fn median_absolute_deviaiton(input: &Input) -> Num {
    let m = median(input);

    let half = input.len() / 2;
    let median = if half % 2 == 1 {
        let mut skipper = input.iter().skip(half);
        let mut n = if let Some(e) = skipper.next() {
            (*e - m).abs()
        } else {
            unreachable!("")
        };
        if let Some(e) = skipper.next() {
            n += (*e - m).abs()
        };

        n / 2 as Num
    } else {
        let n = *input.iter().skip(half).next().unwrap();
        (n - m).abs()
    };
    // abs median
    median
}

pub mod percentile {
    use super::*;
    use std::ops::RangeInclusive;
    
    fn percentile_averageout(input: &Input, index: Num, whole: Num) -> Num {
        let weight = index - whole;
        
        let c1 = input[whole as usize] * (1 as Num - weight);
        let c2 = input[(whole as usize) - 1] * (weight);

        (c1 + c2) / 2 as Num
    }

    pub fn nearest_rank(input: &Input, percent: Num) -> RangeInclusive<usize> {
        let ordinal_rank = (
            input.len() as Num 
            * 
            (percent / 100 as Num).round()
        ) as usize;
        0..=(ordinal_rank+1)
    }
    
    pub fn quartile(input: &Input, percent: Num) -> Num {
        let percentage = (input.len() as Num * percent) / 100 as Num;
        let rounded = percentage.round();

        if rounded == percentage {
            input[percentage as usize]
        } else {
            let weight = 1 as Num - percentage.fract();
            let idx = rounded as usize;
            (input[idx] * weight) + (input[idx + 1] * percentage.fract())
        }
    }

}

pub fn entropy(input: &Input) -> Num {
    let normalized = {
        let mut vec = Vec::with_capacity(input.len());
        let sum = input.iter().fold(0 as Num, |acc, n| {
            vec.push(*n);
            acc + n
        });
        vec.iter_mut().for_each(|i| *i = *i / sum);
        vec
    };

    let r = normalized.iter().fold(0 as Num, |result, n| {
        if result == 0 as Num {
            result
        } else {
            let e = std::f64::consts::E;
            result + n * n.log(e)
        }
    });

    -r
}

pub fn sigmoid(input: &Input) -> Vec<Num> {
    let mut vec = Vec::with_capacity(input.len());
    let one_num = 1 as Num;
    for item in input.iter() {
        vec.push(one_num / (one_num + (-item).exp()))
    }
    vec
}

pub fn soft_max(input: &Input) -> Vec<Num> {
    let max_value = max(input);

    let sum = input.iter().fold(0 as Num, |acc, n| {
        acc + (*n - max_value).exp()
    });

    let mut vec = Vec::with_capacity(input.len());
    for i in input.iter() {
        vec.push((i - max_value) / sum)
    }

    vec
}

#[derive(Debug)]
pub struct Quantile {
    q1: Num,
    q2: Num,
    q3: Num
}

impl Quantile {
    #[inline]
    pub fn new(input: &Input) -> Self {
        quantile(input)
    }
    pub fn inter_quartile_range(&self) -> Num {
        self.q3 - self.q1
    }
    pub fn mid_hind(&self) -> Num {
        (self.q1 + self.q3) / 2 as Num
    }
    pub fn trimean(&self) -> Num {
        (self.q1 + ((self.q2 * 2 as Num) + self.q3)) / 4 as Num
    }

    #[inline]
    pub fn q1(&self) -> Num {
        self.q1
    }

    #[inline]
    pub fn q2(&self) -> Num {
        self.q2
    }

    #[inline]
    pub fn q3(&self) -> Num {
        self.q3
    }
}

#[derive(Debug, PartialEq)]
pub struct Outliers {
    pub extreme: Vec<Num>,
    pub mild: Vec<Num>,
    pub lif: Num,
    pub uif: Num,
    pub lof: Num,
    pub uof: Num
}

pub fn quartile_outliers(input: &Input, q: impl Into<Option<Quantile>>) -> Outliers {
    let q: Quantile = q.into().unwrap_or(quantile(input));

    let iqr = q.inter_quartile_range();

    let one_n_half = 1.5 * iqr;
    let three = 3. * iqr;

    let lif = q.q1 - one_n_half;
    let uif = q.q3 + one_n_half;
    let lof = q.q1 - three;
    let uof = q.q3 + three;

    let cap = input.len()/3;
    let mut extreme = Vec::with_capacity(cap);
    let mut mild = Vec::with_capacity(cap);
    for i in input.iter() {
        let i = *i;

        if i < lof || i > uof {
            extreme.push(i)
        } else if i < lif || i > uif {
            mild.push(i)
        }
    }

    Outliers {
        extreme,
        mild,
        lif,
        uif,
        lof,
        uof
    }
}

pub fn quantile(input: &Input) -> Quantile {
    let f_len = input.len() as Num;
    
    let (range_q1, range_q3) = if f_len % 2 as Num == 0 as Num {
        let half = input.len() / 2;
        (0..half, half..input.len())
    } else {
        let half = (input.len()-1) / 2;
        (0..half, (half+1)..input.len())
    };

    let q1 = median(&input[range_q1]);
    let q2 = median(input);
    let q3 = median(&input[range_q3]);

    Quantile {q1, q2, q3}
}
