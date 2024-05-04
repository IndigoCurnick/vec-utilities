use approx_eq::assert_approx_eq;
use vec_utilities::running::Running;

#[test]
fn test_running_sum() {
    let v = vec![1.0, 2.0, 3.0];

    let running: Vec<f64> = v.iter().running_sum().collect();

    assert_eq!(running, vec![1.0, 3.0, 6.0]);
}

#[test]
fn test_running_mean() {
    let v = vec![1.0, 2.0, 3.0, 4.0];

    let running: Vec<f64> = v.iter().running_mean().collect();

    assert_eq!(running, vec![1.0, 1.5, 2.0, 2.5]);
}

#[test]
fn test_running_std() {
    let v = vec![1.0, 2.0, 3.0, 4.0];

    let running: Vec<f64> = v.iter().running_std().collect();
    let correct = vec![0.0, 0.70710678118655, 1.0, 1.2909944487358];

    for (r, c) in running.iter().zip(correct.iter()) {
        assert_approx_eq!(*r, *c, 1e-8);
    }
}

#[test]
fn test_running_sum_with_filter() {
    let v = vec![1.0, 2.0, 3.0];

    let running: Vec<f64> = v.iter().running_sum().filter(|z| *z > 2.0).collect();

    assert_eq!(running, vec![3.0, 6.0]);
}
