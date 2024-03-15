pub trait Statistics<T> {
    fn mean(self) -> Option<T>;
    fn median(self) -> Option<T>;
    fn variance(self) -> Option<T>;
    fn std(self) -> Option<T>;

    // Called them float_max and float_min so that there's no conflicts
    fn float_max(self) -> T;
    fn float_min(self) -> T;

    fn difference(self) -> T;
    fn zero_crossings(self) -> usize;

    fn peak_average_ratio(self) -> Option<T>;
}

macro_rules! impl_stats_iterator {
    ($float:ty) => {
        impl<'a, T: Iterator<Item = &'a $float>> Statistics<$float> for T {
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
                // TODO: Not 100% happy with this implementation due to the cloning but what can you do?
                let elements: Vec<$float> = self.cloned().collect();
                let len = elements.len();
                if elements.len() == 0 {
                    return None;
                }

                let mut new = elements
                    .iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<$float>>();

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

            fn float_max(self) -> $float {
                self.fold(<$float>::NEG_INFINITY, |a, b| a.max(*b))
            }

            fn float_min(self) -> $float {
                self.fold(<$float>::INFINITY, |a, b| a.min(*b))
            }

            fn difference(self) -> $float {
                // self.float_max() - self.float_min()
                // You'd like to do that but it consumes the iterator

                let mut min = <$float>::INFINITY;
                let mut max = <$float>::NEG_INFINITY;

                for n in self {
                    min = n.min(min);
                    max = n.max(max);
                }

                max - min
            }

            fn zero_crossings(self) -> usize {
                let mut zero_crossings = 0;
                let mut prev_item = None;

                for item in self {
                    if let Some(prev) = prev_item {
                        if prev * item < 0.0 {
                            zero_crossings += 1;
                        }
                    }
                    prev_item = Some(item);
                }

                zero_crossings
            }

            fn peak_average_ratio(self) -> Option<$float> {
                let (sum, count, max) = self.fold(
                    (0.0, 0, <$float>::NEG_INFINITY),
                    |(sum, count, max), item| (sum + item, count + 1, item.max(max)),
                );

                if count == 0 {
                    return None;
                } else {
                    return Some(max / (sum / count as $float));
                }
            }
        }
    };
}

impl_stats_iterator!(f64);
impl_stats_iterator!(f32);
