fn test_value() -> Vec<f64> {
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .map(|i| *i as f64)
        .collect::<Vec<f64>>()
}

#[test]
fn max() {
    let list = test_value();
    assert_eq!(crate::max(&list[..]), 10.);
}

#[test]
fn min() {
    let list = test_value();
    assert_eq!(crate::min(&list[..]), 0.);
}

#[test]
fn mean() {
    assert_eq!(crate::mean(&test_value()[..]), 5.);
    // https://www.investopedia.com/ask/answers/06/geometricmean.asp
    assert_eq!(
        crate::geometric_mean(&[1.9, 1.1, 1.2, 1.3, 0.1]),
        0.7991976107924678
    );
    // https://www.mathsisfun.com/numbers/geometric-mean.html
    assert_eq!(crate::geometric_mean(&[16.; 3]), 15.999999999999998);

    // wikipedia
    assert_eq!(crate::harmonic_mean(&[1., 4., 4.]), 2.);
    // https://www.mathsisfun.com/numbers/harmonic-mean.html
    let trunc = (crate::harmonic_mean(&[1., 2., 4.]) * 1000.) as i64;
    assert_eq!(trunc, 1714);
}

#[test]
pub fn variance() {
    let list = [600., 470., 170., 430., 300.];
    assert_eq!(crate::mean(&list), 394.);
    assert_eq!(crate::population_variance(&list), 21704.);
    assert_eq!(crate::sample_variance(&list).round(), 165.);

    assert_eq!(crate::sample_variance(&list).round(), 165.);
}

#[test]
pub fn deviation() {
    let input = [600., 470., 170., 430., 300.];
    let std = crate::standard_deviation_population(&input[..]);

    assert_eq!(std.round(), 147.);

    let std2 = crate::standard_deviation_sample(&input[..]);
    assert_eq!(std2.round(), 13.);
}

#[test]
pub fn generics() {
    let input = [1, 2, 3, 4, 5]
        .iter()
        .map(|i| *i as f64)
        .collect::<Vec<f64>>();

    let x = crate::quartile_outliers(&input, None);
    let y = crate::quartile_outliers(&input, crate::quantile(&input));

    assert_eq!(x, y);
}
