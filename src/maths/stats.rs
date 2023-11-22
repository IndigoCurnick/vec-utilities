use num_traits::{Float, NumCast};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::str::FromStr;

pub trait Stats<U>
where
    U: Float,
{
    fn mean(&self) -> Option<U>;
    fn nan_mean(&self) -> Option<U>;
    fn median(self) -> Option<U>;
    fn nan_median(&self) -> Option<U>;
    fn mode(&self) -> Option<U>;
    fn nan_mode(&self) -> Option<U>;
    fn variance(&self) -> Option<U>;
    fn nan_variance(&self) -> Option<U>;
    fn std(&self) -> Option<U>;
    fn nan_std(&self) -> Option<U>;
}

impl<T> Stats<T> for Vec<T>
where
    T: Float + Sum + Display + FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    fn mean(&self) -> Option<T> {
        if self.len() > 0 {
            return Some(
                self.iter().cloned().sum::<T>() / <T as NumCast>::from(self.len()).unwrap(),
            );
        } else {
            return None;
        }
    }

    fn nan_mean(&self) -> Option<T> {
        if self.len() > 0 {
            return Some(
                self.iter().filter(|x| !x.is_nan()).cloned().sum::<T>()
                    / <T as NumCast>::from(self.iter().filter(|x| !x.is_nan()).count()).unwrap(),
            );
        } else {
            return None;
        }
    }

    fn median(mut self) -> Option<T> {
        // Median consumes self because we need to sort the vec
        // This means the programmer can choose whether to `.clone().median()` for a performance hit
        // Or `.median()` if they no longer need the `Vec` after this

        let n = self.len();

        if n == 0 {
            return None;
        }

        // See https://doc.rust-lang.org/std/primitive.Float.html#method.total_cmp
        // `total_cmp` has been implemented on f32 and Float since 1.62.0
        self.sort_by(|a, b| {
            <f64 as NumCast>::from(*a)
                .unwrap()
                .total_cmp(&<f64 as NumCast>::from(*b).unwrap())
        });
        let mid_index = n / 2; // Note, this is automatically a floor division because of how Rust usize works
                               // In Python you would do something like `mid_index = n // 2`

        if n % 2 == 1 {
            return Some(self[mid_index]);
        } else {
            return Some(
                (self[mid_index - 1] + self[mid_index + 1]) / <T as NumCast>::from(2.0).unwrap(),
            );
        }
    }

    fn nan_median(&self) -> Option<T> {
        // Unlike median, I think we need to make a new vec in memory here, so
        // there would be no performance benefit of passing by value. Thus,
        // unlike `median`, we take a reference.

        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nans = self
            .iter()
            .filter(|&x| !x.is_nan())
            .cloned()
            .collect::<Vec<T>>();

        return no_nans.median();
    }

    fn mode(&self) -> Option<T> {
        fn insert_map(num: String, m: &mut HashMap<String, usize>) {
            if let Some(x) = m.get_mut(&num) {
                *x += 1;
            } else {
                m.insert(num, 1);
            }
        }

        if self.len() == 0 {
            return None;
        }

        let mut m: HashMap<String, usize> = HashMap::new();

        self.iter().for_each(|x| insert_map(x.to_string(), &mut m));

        let mut mode_float = "".to_string();
        let mut mode_count = 0;

        for (k, v) in m.iter() {
            if v > &mode_count {
                mode_float = k.clone();
                mode_count = *v;
            }
        }

        return Some(mode_float.parse::<T>().unwrap());
    }

    fn nan_mode(&self) -> Option<T> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nans = self
            .iter()
            .filter(|&x| !x.is_nan())
            .cloned()
            .collect::<Vec<T>>();

        return no_nans.mode();
    }

    fn variance(&self) -> Option<T> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let mean = self.mean()?;

        return Some(
            self.iter()
                .map(|x| (*x - mean).powf(<T as NumCast>::from(2.0).unwrap()))
                .sum::<T>()
                / <T as NumCast>::from(n).unwrap(),
        );
    }

    fn nan_variance(&self) -> Option<T> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nan: Vec<T> = self.iter().filter(|x| !x.is_nan()).cloned().collect();

        return no_nan.variance();
    }

    fn std(&self) -> Option<T> {
        return match self.variance() {
            Some(x) => Some(x.powf(<T as NumCast>::from(0.5).unwrap())),
            None => None,
        };
    }

    fn nan_std(&self) -> Option<T> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nan: Vec<T> = self.iter().filter(|x| !x.is_nan()).cloned().collect();

        return match no_nan.variance() {
            Some(x) => Some(x.powf(<T as NumCast>::from(0.5).unwrap())),
            None => None,
        };
    }
}
