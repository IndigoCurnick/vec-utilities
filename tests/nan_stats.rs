use vec_utilities::maths::nan_stats::NanStatistics;

#[test]
fn test_nan_median() {
    let test_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, f64::NAN];

    let median = test_vec.iter().nan_median();

    assert_eq!(median.unwrap(), 3.0);
}

#[test]
fn test_nan_variance() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f64::NAN];

    let var = test_vec.iter().nan_variance();

    assert_eq!(var.unwrap(), 4.0);
}

#[test]
fn test_nan_std() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f64::NAN];

    let std = test_vec.iter().nan_std();

    assert_eq!(std.unwrap(), 2.0);

    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f32::NAN];

    let std = test_vec.iter().nan_std();

    assert_eq!(std.unwrap(), 2.0);
}

#[test]
fn test_nan_mean() {
    let test_vec = vec![2.0, 2.0, 2.0, f64::NAN];

    let mean = test_vec.iter().nan_mean();

    assert_eq!(mean.unwrap(), 2.0);
}
