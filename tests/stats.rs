use vec_utilities::maths::stats::Statistics;
use vec_utilities::maths::stats::Stats;
#[test]
fn test_mean() {
    let test_vec = vec![2.0, 2.0, 2.0];

    let mean = test_vec.mean();

    assert_eq!(mean.unwrap(), 2.0)
}

#[test]
fn test_nan_mean() {
    let test_vec = vec![2.0, 2.0, 2.0, f64::NAN];

    let mean = test_vec.nan_mean();

    assert_eq!(mean.unwrap(), 2.0);
}

#[test]
fn test_median() {
    let test_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let median = test_vec.median();

    assert_eq!(median.unwrap(), 3.0);
}

#[test]
fn test_nan_median() {
    let test_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, f64::NAN];

    let median = test_vec.nan_median();

    assert_eq!(median.unwrap(), 3.0);
}

#[test]
fn test_variance() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let var = test_vec.variance();

    assert_eq!(var.unwrap(), 4.0);
}

#[test]
fn test_std() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let std = test_vec.std();

    assert_eq!(std.unwrap(), 2.0);
}

#[test]
fn test_nan_variance() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f64::NAN];

    let var = test_vec.nan_variance();

    assert_eq!(var.unwrap(), 4.0);
}

#[test]
fn test_nan_std() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f64::NAN];

    let std = test_vec.nan_std();

    assert_eq!(std.unwrap(), 2.0);

    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f32::NAN];

    let std = test_vec.nan_std();

    assert_eq!(std.unwrap(), 2.0);
}

#[test]
fn test_macro_mean() {
    let x = vec![2.0, 3.0, 2.0, 3.0];
    let y = vec![2.0, 3.0, 2.0, 3.0, f64::NAN];

    let iter_mean = x.iter().mean();
    let into_iter_mean = x.into_iter().mean();

    let nan_mean = y.iter().nan_filter().iter().mean();

    println!(
        "Iter mean: {}\nInto iter mean:{}\nNaN Mean: {}",
        iter_mean.unwrap(),
        into_iter_mean.unwrap(),
        nan_mean.unwrap()
    )
}
#[test]
fn test_macro_variance() {
    let x = vec![2.0, 3.0, 2.0, 3.0];
    let y = vec![2.0, 3.0, 2.0, 3.0, f64::NAN];

    let iter_var = x.iter().variance();
    let into_iter_var = x.into_iter().variance();

    let nan_var = y.iter().nan_filter().iter().variance();

    println!(
        "Iter var: {}\nInto iter var:{}\nNaN var: {}",
        iter_var.unwrap(),
        into_iter_var.unwrap(),
        nan_var.unwrap()
    )
}

#[test]
fn test_macro_median() {
    let x = vec![2.0, 3.0, 2.0, 3.0];
    let y = vec![2.0, 3.0, 2.0, 3.0, f64::NAN];

    let iter_var = x.iter().median();
    let into_iter_var = x.into_iter().median();

    let nan_var = y.iter().nan_filter().iter().median();

    println!(
        "Iter var: {}\nInto iter var:{}\nNaN var: {}",
        iter_var.unwrap(),
        into_iter_var.unwrap(),
        nan_var.unwrap()
    )
}
