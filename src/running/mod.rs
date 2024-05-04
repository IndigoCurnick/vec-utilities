// See https://nullbuffer.com/articles/welford_algorithm.html

use num::Float;

// TODO: These would actually be way better as iterators

// Running Sum
pub struct RunningSum<'a, T: Float + 'a, I: Iterator<Item = &'a T>> {
    iter: I,
    sum: T,
}

impl<'a, T: Float + 'a, I: Iterator<Item = &'a T>> RunningSum<'a, T, I> {
    pub fn new(iter: I) -> Self {
        return Self {
            iter: iter,
            sum: T::zero(),
        };
    }
}

impl<'a, T: Float + 'a, I: Iterator<Item = &'a T>> Iterator for RunningSum<'a, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => {
                self.sum = self.sum + *x;

                return Some(self.sum);
            }
            None => return None,
        }
    }
}

// Running Mean
pub struct RunningMean<'a, T: Float + 'a, I: Iterator<Item = &'a T>> {
    iter: I,
    m: T,
    k: T,
}

impl<'a, T: Float + 'a, I: Iterator<Item = &'a T>> RunningMean<'a, T, I> {
    pub fn new(iter: I) -> Self {
        return Self {
            iter,
            m: T::zero(),
            k: T::one(),
        };
    }
}

impl<'a, T: Float + 'a, I: Iterator<Item = &'a T>> Iterator for RunningMean<'a, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => {
                self.m = self.m + (*x - self.m) / self.k;
                self.k = self.k + T::one();

                return Some(self.m);
            }
            None => return None,
        }
    }
}

// Running Std
pub struct RunningStd<'a, T: Float + 'a, I: Iterator<Item = &'a T>> {
    iter: I,
    n: T,
    mean: T,
    m2: T,
}

impl<'a, T: Float + 'a, I: Iterator<Item = &'a T>> RunningStd<'a, T, I> {
    pub fn new(iter: I) -> Self {
        return Self {
            iter,
            n: T::zero(),
            mean: T::zero(),
            m2: T::zero(),
        };
    }
}

impl<'a, T: Float + 'a, I: Iterator<Item = &'a T>> Iterator for RunningStd<'a, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => {
                self.n = self.n + T::one();
                let delta = *x - self.mean;
                self.mean = self.mean + (delta / self.n);
                let delta2 = *x - self.mean;
                self.m2 = self.m2 + (delta * delta2);

                if self.n < T::from(2.0).unwrap() {
                    return Some(T::zero());
                } else {
                    return Some((self.m2 / (self.n - T::one())).sqrt());
                }
            }
            None => return None,
        }
    }
}

// Use the `Running` trait
pub trait Running<'a, T: Float + 'a, I: Iterator<Item = &'a T>> {
    fn running_sum(self) -> RunningSum<'a, T, I>;
    fn running_mean(self) -> RunningMean<'a, T, I>;
    fn running_std(self) -> RunningStd<'a, T, I>;
}

impl<'a, T: Float + 'a, I: Iterator<Item = &'a T>> Running<'a, T, I> for I {
    fn running_sum(self) -> RunningSum<'a, T, I> {
        RunningSum::new(self)
    }

    fn running_mean(self) -> RunningMean<'a, T, I> {
        RunningMean::new(self)
    }

    fn running_std(self) -> RunningStd<'a, T, I> {
        RunningStd::new(self)
    }
}
