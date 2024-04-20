pub trait Filters<T> {
    fn filter_nans(&self) -> Vec<T>;
}

macro_rules! impl_filter {
    ($float:ty) => {
        impl Filters<$float> for Vec<$float> {
            fn filter_nans(&self) -> Vec<$float> {
                return self.iter().cloned().filter(|x| !x.is_nan()).collect();
            }
        }
    };
}

impl_filter!(f32);
impl_filter!(f64);
