use ta_common::traits::Indicator;

#[doc(include = "../README.md")]
pub struct Wilders {
    period: u32,
    prev_avg: f64,
    sum: f64,
    index: u32,
}

impl Wilders {
    pub fn new(period: u32) -> Wilders {
        Self {
            period,
            prev_avg: 0.0,
            sum: 0.0,
            index: 0,
        }
    }
}

impl Indicator<f64, Option<f64>> for Wilders {
    fn next(&mut self, input: f64) -> Option<f64> {
        self.index = self.index + 1;
        println!("index {} sum {}", self.index, self.sum);

        if self.index < self.period {
            self.sum = self.sum + input;
            return None;
        }
        if self.index == (self.period) {
            self.sum = self.sum + input;
            let res = self.sum / self.period as f64;
            println!("index {} sum {}", self.index, self.sum);
            self.prev_avg = res;
            return Some(res);
        }

        let period = self.period as f64;
        let res = (self.prev_avg * (period - 1.0) + input) / period;
        self.prev_avg = res;
        Some(res)
    }

    fn reset(&mut self) {
        self.sum = 0.0;
        self.index = 0;
        self.prev_avg = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::Wilders;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut wilders = Wilders::new(5);
        assert_eq!(wilders.next(81.59), None);
        assert_eq!(wilders.next(81.06), None);
        assert_eq!(wilders.next(82.87), None);
        assert_eq!(wilders.next(83.00), None);
        assert_eq!(wilders.next(83.61), Some(82.426));
        assert_eq!(wilders.next(83.15), Some(82.5708));
        assert_eq!(wilders.next(82.84), Some(82.62464));
        assert_eq!(wilders.next(83.99), Some(82.897712));
        assert_eq!(wilders.next(84.55), Some(83.2281696));
        assert_eq!(wilders.next(84.36), Some(83.45453568));
        assert_eq!(wilders.next(85.53), Some(83.86962854400001));
        assert_eq!(wilders.next(86.54), Some(84.40370283520001));
        assert_eq!(wilders.next(86.89), Some(84.90096226816));
        assert_eq!(wilders.next(87.77), Some(85.474769814528));
        assert_eq!(wilders.next(87.29), Some(85.83781585162241));
    }
}
