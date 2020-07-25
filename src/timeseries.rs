use crate::*;

pub mod moving_average {
    use super::*;

    pub fn simple(input: &Input, n: usize) -> Vec<Num> {
        let n_scale = n as Num;
        user_original(input, n, |slice| sum(&slice) / n_scale)
    }

    pub fn weighted(input: &Input, n: usize) -> Vec<Num> {
        user_original(input, n, |slice| {
            slice
                .iter()
                .enumerate()
                .fold(0 as Num, |acc, (idx, n)| acc + n / (idx as Num))
        })
    }

    /// slice of a input is passed onto the callback
    /// slice is \sigma _ i=0 input[i..(i+n)]
    pub fn user_original<F>(input: &Input, n: usize, func: F) -> Vec<Num>
    where
        F: Fn(&Input) -> Num,
    {
        let scale = n;
        let mut vec = Vec::with_capacity(input.len() / scale);

        for i in 0..input.len() {
            let end = i + scale;
            if input.get(end).is_none() {
                break;
            } else {
                vec.push(func(&input[i..end]));
            }
        }

        vec
    }
}
