use std::collections::HashMap;

pub trait Stats {
    fn mean(&self) -> Option<f64>;
    fn nan_mean(&self) -> Option<f64>;
    fn median(self) -> Option<f64>;
    fn nan_median(&self) -> Option<f64>;
    fn mode(&self) -> Option<f64>;
    fn nan_mode(&self) -> Option<f64>;
    fn variance(&self) -> Option<f64>;
    fn nan_variance(&self) -> Option<f64>;
    fn std(&self) -> Option<f64>;
    fn nan_std(&self) -> Option<f64>;
}

impl Stats for Vec<f64> {
    fn mean(&self) -> Option<f64> {
        if self.len() > 0 {
            return Some(self.iter().sum::<f64>() / self.len() as f64);
        } else {
            return None;
        }
    }

    fn nan_mean(&self) -> Option<f64> {
        if self.len() > 0 {
            return Some(
                self.iter().filter(|x| !x.is_nan()).sum::<f64>()
                    / self.iter().filter(|x| !x.is_nan()).count() as f64,
            );
        } else {
            return None;
        }
    }

    fn median(mut self) -> Option<f64> {
        // Median consumes self because we need to sort the vec
        // This means the programmer can choose whether to `.clone().median()` for a performance hit
        // Or `.median()` if they no longer need the `Vec` after this

        let n = self.len();

        if n == 0 {
            return None;
        }

        // See https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
        // `total_cmp` has been implemented on f32 and f64 since 1.62.0
        self.sort_by(|a, b| a.total_cmp(b));
        let mid_index = n / 2; // Note, this is automatically a floor division because of how Rust usize works
                               // In Python you would do something like `mid_index = n // 2`

        if n % 2 == 1 {
            return Some(self[mid_index]);
        } else {
            return Some((self[mid_index - 1] + self[mid_index + 1]) / 2.0);
        }
    }

    fn nan_median(&self) -> Option<f64> {
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
            .collect::<Vec<f64>>();

        return no_nans.median();
    }

    fn mode(&self) -> Option<f64> {
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

        return Some(mode_float.parse::<f64>().unwrap());
    }

    fn nan_mode(&self) -> Option<f64> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nans = self
            .iter()
            .filter(|&x| !x.is_nan())
            .cloned()
            .collect::<Vec<f64>>();

        return no_nans.mode();
    }

    fn variance(&self) -> Option<f64> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let mean = self.mean()?;

        return Some(self.iter().map(|x| (x - mean).powf(2.0)).sum::<f64>() / n as f64);
    }

    fn nan_variance(&self) -> Option<f64> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nan: Vec<f64> = self.iter().filter(|x| !x.is_nan()).cloned().collect();

        return no_nan.variance();
    }

    fn std(&self) -> Option<f64> {
        return match self.variance() {
            Some(x) => Some(x.powf(0.5)),
            None => None,
        };
    }

    fn nan_std(&self) -> Option<f64> {
        let n = self.len();

        if n == 0 {
            return None;
        }

        let no_nan: Vec<f64> = self.iter().filter(|x| !x.is_nan()).cloned().collect();

        return match no_nan.variance() {
            Some(x) => Some(x.powf(0.5)),
            None => None,
        };
    }
}
