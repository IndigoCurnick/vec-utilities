use vec_utilities::running::Running;

#[test]
fn test_running_sum() {
    let v = vec![1.0, 2.0, 3.0];

    let running = v.iter().running_sum();

    assert_eq!(running, vec![1.0, 3.0, 6.0]);
}

#[test]
fn test_running_mean() {
    let v = vec![1.0, 2.0, 3.0, 4.0];

    let running = v.iter().running_mean();

    assert_eq!(running, vec![1.0, 1.5, 2.0, 2.5]);
}
