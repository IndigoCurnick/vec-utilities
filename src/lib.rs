mod maths;
use crate::maths::Stats;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean() {
        let test_vec = vec![2.0, 2.0, 2.0, f64::NAN];

        let mean = test_vec.mean();

        assert_eq!(mean.unwrap(), 2.0)
    }
}
