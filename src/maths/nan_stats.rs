use crate::maths::stats::Statistics;

/// Use this trait if you suspect you might have NaNs in your Vec and you want to
/// ignore them for the computations
pub trait NanStatistics<T> {
    fn nan_mean(self) -> Option<T>;
    fn nan_variance(self) -> Option<T>;
    fn nan_std(self) -> Option<T>;
    fn nan_median(self) -> Option<T>;
    fn nan_max(self) -> T;
    fn nan_min(self) -> T;

    fn nan_difference(self) -> T;
    fn nan_zero_crossings(self) -> usize;
    fn nan_peak_average_ratio(self) -> Option<T>;
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

            fn nan_difference(self) -> $float {
                self.filter(|x| !x.is_nan()).difference()
            }

            fn nan_zero_crossings(self) -> usize {
                self.filter(|x| !x.is_nan()).zero_crossings()
            }

            fn nan_peak_average_ratio(self) -> Option<$float> {
                self.filter(|x| !x.is_nan()).peak_average_ratio()
            }
        }
    };
}

impl_nan_stat!(f64);
impl_nan_stat!(f32);
