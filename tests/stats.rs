use vec_utilities::maths::stats::Statistics;

#[test]
fn test_mean() {
    let test_vec: Vec<f64> = vec![2.0, 2.0, 2.0];

    let mean = test_vec.iter().mean();

    assert_eq!(mean.unwrap(), 2.0)
}

#[test]
fn test_median() {
    let test_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let median = test_vec.iter().median();

    assert_eq!(median.unwrap(), 3.0);
}

#[test]
fn test_variance() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let var = test_vec.iter().variance();

    assert_eq!(var.unwrap(), 4.0);
}

#[test]
fn test_std() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let std = test_vec.iter().std();

    assert_eq!(std.unwrap(), 2.0);
}

#[test]
fn test_max() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let max = test_vec.iter().float_max();

    assert_eq!(max, 9.0);
}

#[test]
fn test_min() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let min = test_vec.iter().float_min();

    assert_eq!(min, 2.0);
}

#[test]
fn test_difference() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let diff = test_vec.iter().difference();

    assert_eq!(diff, 7.0);
}

#[test]
fn test_zero_crossings() {
    let test_vec = vec![-1.0, 2.0, -3.0, 6.0, 0.0];

    let zc = test_vec.iter().zero_crossings();

    assert_eq!(zc, 3);
}
