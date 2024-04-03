// See https://nullbuffer.com/articles/welford_algorithm.html

pub trait Running<T> {
    fn running_sum(self) -> Vec<T>;
    fn running_mean(self) -> Vec<T>;
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

            fn running_mean(self) -> Vec<$float> {
                let mut m = 0.0;
                let mut k = 1.0;

                let mut out = vec![];

                for m_k in self {
                    m = m + (*m_k - m) / k;
                    k += 1.0;
                    out.push(m);
                }

                return out;
            }
        }
    };
}

impl_running_iterator!(f64);
impl_running_iterator!(f32);
