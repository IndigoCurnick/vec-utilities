pub trait Stats {
    fn mean(&self) -> Option<f64>;
    // fn nanmean(&self) -> Option<f64>;
}

impl Stats for Vec<f64> {
    fn mean(&self) -> Option<f64> {
        if self.len() > 0 {
            return Some(self.iter().sum::<f64>() / self.len() as f64);
        } else {
            return None;
        }
    }

    // fn nanmean(&self) -> Option<f64> {
    //     if self.len() > 0 {
    //         return Some(
    //             self.iter()
    //                 .filter(|x| !x.is_nan())
    //                 .collect::<Vec<f64>>()
    //                 .sum::<f64>()
    //                 / self.len() as f64,
    //         );
    //     } else {
    //         return None;
    //     }
    // }
}
