/// functions that takes two argument will be found hereuuse crate::generic_types::*;
use crate::generic_types::*;
use crate::uni_dimentional::*;
use std::f64::consts::E as NumE;

pub fn correlation(input1: &Input, input2: &Input) -> Num {
    let sdev1 = standard_deviation_population(input1);
    let sdev2 = standard_deviation_population(input2);

    let zero = 0 as Num;

    if [sdev1, sdev2].contains(&zero) {
        0 as Num
    } else {
        let covp = covariance_population(input1, input2);
        covp / (sdev1 * sdev2)
    }
}

pub fn auto_correlation(input: &Input, lags: Int) -> Num {
    let avg = mean(input);

    let first_value = (input[0] - avg).powi(2);

    let mut result = 0 as Num;
    let mut q = 0 as Num;

    for _ in 0..lags {
        let mut v = first_value;

        for i in 1..input.len() {
            let d1 = input[i] - avg;
            let d2 = input[i - 1] - avg;

            let float = (i + 1) as Num;
            q += (d1 * d2 - q) / float;
            v += (d1 * d1 - v) / float;
        }

        result = q / v;
    }

    result
}
#[derive(Debug, Default)]
pub struct LinearRegression<'a> {
    pub gradient: Num,
    pub intercept: Num,
    pub input_x: Option<&'a Input>,
}

impl<'a> LinearRegression<'a> {
    pub fn build_y(&self) -> Vec<Num> {
        self.input_x
            .unwrap()
            .iter()
            .map(|i| (*i * self.gradient) + self.intercept)
            .collect::<Vec<Num>>()
    }
    pub fn set_input_x(&mut self, input_x: &'a Input) {
        self.input_x.replace(input_x);
    }
}

/// calculates intercept and gradient of 2 inputs
pub fn linear_regression<'a>(input_x: &'a Input, input_y: &Input) -> LinearRegression<'a> {
    let mut sum = [0 as Num; 5];
    for i in 0..input_x.len() {
        let x = input_x[i];
        let y = input_y[i];
        [x, y, x * x, x * y, y * y]
            .iter()
            .enumerate()
            .for_each(|(i, item)| {
                sum[i] += item;
            })
    }

    let float_len = input_x.len() as Num;
    let gradient = (float_len * sum[3] - sum[0] * sum[1]) / (float_len * sum[2] - sum[0] * sum[0]);
    let intercept = (sum[1] / float_len) - (gradient * sum[0] / float_len);

    LinearRegression {
        gradient,
        intercept,
        input_x: Some(input_x),
    }
}

pub struct ExponentialRegression<'a> {
    pub a: Num,
    pub b: Num,
    pub input_x: Option<&'a Input>,
}

impl<'a> ExponentialRegression<'a> {
    pub fn build_y(&self) -> Vec<Num> {
        self.bulid_y_with_logarithm(NumE)
    }
    pub fn bulid_y_with_logarithm(&self, logarithm: Num) -> Vec<Num> {
        let a = self.a;
        let b = self.b;
        self.input_x
            .unwrap_or(&[0.])
            .iter()
            .map(|i| a * logarithm.log(b * i))
            .collect::<Vec<Num>>()
    }
}

pub fn exponential_regression<'a>(
    input_x: &'a Input,
    input_y: &Input,
) -> ExponentialRegression<'a> {
    let mut sum = [0 as Num; 5];
    for i in 0..input_x.len() {
        let x = input_x[i];
        let y = input_y[i];
        let y_pow2_log = y * y.log(NumE);
        let xy_mul = x * y;
        [y, x * xy_mul, y_pow2_log, x * y_pow2_log, xy_mul]
            .iter()
            .enumerate()
            .for_each(|(i, item)| {
                sum[i] += item;
            })
    }

    let sum_4_pow2 = sum[4] * sum[4];
    let denominator = sum[0] * sum[1] - sum_4_pow2;
    let a = NumE.powf((sum[1] * sum[2] - sum[4] * sum[3]) / denominator);
    let b = ((sum[0] * sum[3]) - sum_4_pow2) / denominator;

    ExponentialRegression {
        a,
        b,
        input_x: Some(input_x),
    }
}

pub fn logarithm_regression<'a>(input_x: &'a Input, input_y: &Input) -> ExponentialRegression<'a> {
    let mut sum = [0 as Num; 4];
    for i in 0..input_x.len() {
        let x = input_x[i];
        let y = input_y[i];
        let x_log = x.log(NumE);
        [x_log, y * x_log, y, x_log.powf(2 as Num)]
            .iter()
            .enumerate()
            .for_each(|(i, item)| {
                sum[i] += item;
            })
    }

    let float_len = input_x.len() as Num;
    let a = (float_len * sum[1] - sum[2] * sum[0]) / (float_len * sum[3] - sum[0] * sum[0]);
    let b = (sum[2] - a * sum[0]) / float_len;

    ExponentialRegression {
        a,
        b,
        input_x: Some(input_x),
    }
}

fn distance_base<F>(input1: &Input, input2: &Input, func: F) -> Num
where
    F: Fn(Num, Num) -> Num,
{
    let mut distance = 0 as Num;
    for i in 0..input1.len() {
        let dif = input1[i] - input2[i];
        distance = func(dif, distance);
    }
    distance
}

pub fn euclidean_distance(input1: &Input, input2: &Input) -> Num {
    let dis = distance_base(input1, input2, |dif, distance| distance + dif * dif);
    dis.sqrt()
}

pub fn chebyshev_distance(input1: &Input, input2: &Input) -> Num {
    let dis = distance_base(input1, input2, |dif, distance| {
        if let Some(std::cmp::Ordering::Less) = distance.partial_cmp(&dif.abs()) {
            dif
        } else {
            distance
        }
    });

    dis
}

pub fn manhattan_distance(input1: &Input, input2: &Input) -> Num {
    distance_base(input1, input2, |dif, dis| dis + dif.abs())
}

pub fn minkowski_distance(input1: &Input, input2: &Input, city_blocks: Num) -> Num {
    let dis = distance_base(input1, input2, |dif, dis| dis + dif.abs().powf(city_blocks));

    dis.powf(1 as Num / city_blocks)
}

pub fn covariance_population(input1: &Input, input2: &Input) -> Num {
    let m1 = mean(input1);
    let m2 = mean(input2);

    let sum = (0..input1.len()).fold(0 as Num, |acc, i| {
        let d1 = &input1[i] - m1;
        let d2 = &input2[i] - m2;
        acc + d1 * d2
    });

    sum / (input1.len() as Num)
}

pub fn sample_covariance(input1: &Input, input2: &Input) -> Num {
    let m1 = mean(input1);
    let m2 = mean(input2);

    let mut s = 0 as Num;
    for i in 0..input1.len() {
        let d1 = &input1[i] - m1;
        let d2 = &input2[i] - m2;
        s += d1 * d2;
    }

    let l = input1.len();

    s * (l as Num) / ((l - 1) as Num)
}
