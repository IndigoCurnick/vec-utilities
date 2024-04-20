use vec_utilities::generation::arange;

#[test]
fn test_arange() {
    let x = arange(0.0, 5.0, 1.0).unwrap();
    let correct = vec![0.0, 1.0, 2.0, 3.0, 4.0];

    assert_eq!(x, correct);
}
