use core::slice::Iter;
use num_traits::float::TotalOrder;
use num_traits::Float;
use std::iter::Filter;
use std::vec::IntoIter;
pub trait Stats<T>
where
    T: Float + TotalOrder,
{
    fn mean(&self) -> Option<T>;
    fn variance(&self) -> Option<T>;
    fn std(&self) -> Option<T>;
    fn median(&self) -> Option<T>;
    fn nan_mean(&self) -> Option<T>;
    fn nan_variance(&self) -> Option<T>;
    fn nan_std(&self) -> Option<T>;
    fn nan_median(&self) -> Option<T>;
}

impl<T> Stats<T> for Vec<T>
where
    T: Float + TotalOrder,
{
    fn mean(&self) -> Option<T> {
        let mut n = T::zero();
        let mut sum = T::zero();
        for item in self {
            n = n.add(T::one());
            sum = sum.add(*item);
        }

        if n > T::zero() {
            return Some(sum / n);
        } else {
            return None;
        }
    }

    fn variance(&self) -> Option<T> {
        // Use Welford's method
        // https://jonisalonen.com/2013/deriving-welfords-method-for-computing-variance/
        let mut m = T::zero();
        let mut s = T::zero();
        let mut k = T::one();

        for item in self {
            let old_m = m;
            m = m + (*item - m) / k;
            s = s + (*item - m) * (*item - old_m);
            k = k + T::one();
        }

        if k > T::zero() {
            return Some(s / (k - T::one()));
        } else {
            return None;
        }
    }

    fn std(&self) -> Option<T> {
        return match self.variance() {
            Some(x) => Some(x.sqrt()),
            None => None,
        };
    }

    fn median(&self) -> Option<T> {
        let len = self.len();
        if self.len() == 0 {
            return None;
        }

        let mut new = self.clone();

        new.sort_by(|a, b| a.total_cmp(b));

        let mid = len / 2;

        if len % 2 == 0 {
            let low = match new.get(mid - 1) {
                Some(x) => x,
                None => return None,
            };

            let high = match new.get(mid) {
                Some(x) => x,
                None => return None,
            };

            return Some((low.add(*high)) / (T::one() + T::one()));
        } else {
            return match new.get(mid) {
                Some(x) => Some(*x),
                None => None,
            };
        }
    }

    fn nan_mean(&self) -> Option<T> {
        let filter = filter_nan(&self);
        return filter.mean();
    }

    fn nan_variance(&self) -> Option<T> {
        let filter = filter_nan(&self);
        return filter.variance();
    }

    fn nan_std(&self) -> Option<T> {
        let filter = filter_nan(&self);
        return filter.std();
    }

    fn nan_median(&self) -> Option<T> {
        let filter = filter_nan(&self);
        return filter.median();
    }
}

fn filter_nan<T>(a: &Vec<T>) -> Vec<T>
where
    T: Float,
{
    return a.iter().filter(|x| !x.is_nan()).cloned().collect();
}

pub trait Statistics<T> {
    fn mean(self) -> Option<T>;
    fn median(self) -> Option<T>;
    fn variance(self) -> Option<T>;
    fn std(self) -> Option<T>;
    fn nan_filter(self) -> Vec<T>;
}

macro_rules! impl_mean {
    ($float:ty, $iter:ty) => {
        impl Statistics<$float> for $iter {
            fn mean(self) -> Option<$float> {
                let (sum, count) =
                    self.fold((0.0, 0), |(sum, count), item| (sum + item, count + 1));

                if count > 0 {
                    Some(sum / (count as $float))
                } else {
                    None
                }
            }
            //TODO! Population implemented here. Sample may be wanted?
            fn variance(self) -> Option<$float> {
                let mut m = 0.0 as $float;
                let mut s = 0.0 as $float;
                let mut k = 1.0 as $float;

                for item in self {
                    let old_m = m;
                    m = m + (item - m) / k;
                    s = s + (item - m) * (item - old_m);
                    k = k + 1.0 as $float;
                }

                if k > 0.0 as $float {
                    return Some(s / (k - 1.0 as $float));
                } else {
                    return None;
                }
            }
            fn std(self) -> Option<$float> {
                self.variance().map(|x| x.sqrt())
            }

            fn median(self) -> Option<$float> {
                let len = self.len();
                if self.len() == 0 {
                    return None;
                }

                let mut new = self.map(|x| x.to_owned()).collect::<Vec<$float>>();

                new.sort_by(|a, b| a.total_cmp(b));

                let mid = len / 2;

                if len % 2 == 0 {
                    let low = match new.get(mid - 1) {
                        Some(x) => x,
                        None => return None,
                    };

                    let high = match new.get(mid) {
                        Some(x) => x,
                        None => return None,
                    };

                    return Some((low + high) / (2.0 as $float));
                } else {
                    return match new.get(mid) {
                        Some(x) => Some(*x),
                        None => None,
                    };
                }
            }

            fn nan_filter(self) -> Vec<$float> {
                self.filter(|x| !x.is_nan()).map(|x| x.to_owned()).collect()
            }
        }
    };
}

impl_mean!(f64, IntoIter<f64>);
impl_mean!(f32, IntoIter<f32>);
impl_mean!(f64, Iter<'_, f64>);
impl_mean!(f32, Iter<'_, f32>);
