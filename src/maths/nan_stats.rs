use crate::maths::stats::Statistics;

pub trait NanStatistics<T> {
    fn nan_mean(self) -> Option<T>;
    fn nan_variance(self) -> Option<T>;
    fn nan_std(self) -> Option<T>;
    fn nan_median(self) -> Option<T>;
    fn nan_max(self) -> T;
    fn nan_min(self) -> T;
}

macro_rules! impl_nan_stat {
    ($float:ty) => {
        impl<'a, T: Iterator<Item = &'a $float>> NanStatistics<$float> for T {
            fn nan_mean(self) -> Option<$float> {
                self.filter(|x| !x.is_nan()).mean()
            }

            fn nan_variance(self) -> Option<$float> {
                self.filter(|x| !x.is_nan()).variance()
            }

            fn nan_std(self) -> Option<$float> {
                self.filter(|x| !x.is_nan()).std()
            }

            fn nan_median(self) -> Option<$float> {
                self.filter(|x| !x.is_nan()).median()
            }

            fn nan_max(self) -> $float {
                self.filter(|x| !x.is_nan()).float_max()
            }

            fn nan_min(self) -> $float {
                self.filter(|x| !x.is_nan()).float_min()
            }
        }
    };
}

impl_nan_stat!(f64);
impl_nan_stat!(f32);
