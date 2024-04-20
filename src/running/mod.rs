// See https://nullbuffer.com/articles/welford_algorithm.html

// TODO: These would actually be way better as iterators
pub trait Running<T> {
    fn running_sum(self) -> Vec<T>;
    fn running_mean(self) -> Vec<T>;
    fn running_std(self) -> Vec<T>;
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

            fn running_std(self) -> Vec<$float> {
                let mut n = 0.0;
                let mut mean = 0.0;
                let mut m2 = 0.0;

                let mut stds = vec![];

                for x in self {
                    n += 1.0;
                    let delta = x - mean;
                    mean += delta / n;
                    let delta2 = x - mean;
                    m2 += delta * delta2;

                    if n < 2.0 {
                        stds.push(0.0);
                    } else {
                        stds.push((m2 / (n - 1.0)).sqrt());
                    }
                }

                return stds;
            }
        }
    };
}

impl_running_iterator!(f64);
impl_running_iterator!(f32);
