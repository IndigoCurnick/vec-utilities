use vec_utilities::running::Running;

#[test]
fn test_running_sum() {
    let v = vec![1.0, 2.0, 3.0];

    let running = v.iter().running_sum();

    assert_eq!(running, vec![1.0, 3.0, 6.0]);
}
