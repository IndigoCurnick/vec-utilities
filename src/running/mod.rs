pub trait Running<T> {
    fn running_sum(self) -> Vec<T>;
}

macro_rules! impl_running_iterator {
    ($float:ty) => {
        impl<'a, T: Iterator<Item = &'a $float>> Running<$float> for T {
            fn running_sum(self) -> Vec<$float> {
                let mut sum = 0.0;
                let mut out = vec![];
                for val in self {
                    sum += val;
                    out.push(sum);
                }

                return out;
            }
        }
    };
}

impl_running_iterator!(f64);
impl_running_iterator!(f32);
