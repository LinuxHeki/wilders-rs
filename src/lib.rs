#![feature(external_doc)]
use ta_common::traits::Indicator;
#[doc(include = "../README.md")]
pub struct Wilders {
    period: u32,
    prev: f64,
    sum: f64,
    index: u32,
}

impl Wilders {
    pub fn new(period: u32) -> Wilders {
        Self {
            period,
            prev: 0.0,
            sum: 0.0,
            index: 0,
        }
    }
}

impl Indicator<f64, Option<f64>> for Wilders {
    fn next(&mut self, input: f64) -> Option<f64> {
        self.index = self.index + 1;
        println!("index {} sum {}",self.index,self.sum);

        if self.index < self.period {
            self.sum = self.sum + input;
            return None;
        }
        if self.index == (self.period) {
            self.sum = self.sum + input;
            self.prev = self.sum / self.period as f64;
            println!("index {} sum {}",self.index,self.sum);
            Some(self.prev);
        }
        self.sum = self.sum - self.prev + input;
        self.prev = self.sum / self.period as f64;
        Some(self.prev)
    }

    fn reset(&mut self) {
        self.prev = 0.0;
        self.sum = 0.0;
        self.index = 0;
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
        assert_eq!(wilders.next(83.61), Some(82.6628));
        assert_eq!(wilders.next(83.15), Some(82.76024));
        assert_eq!(wilders.next(82.84), Some(82.776192));
        assert_eq!(wilders.next(83.99), Some(83.0189536));
        assert_eq!(wilders.next(84.55), Some(83.32516288000001));
        assert_eq!(wilders.next(84.36), Some(83.532130304));
        assert_eq!(wilders.next(85.53), Some(83.93170424320002));
        assert_eq!(wilders.next(86.54), Some(84.45336339456001));
        assert_eq!(wilders.next(86.89), Some(84.940690715648));
        assert_eq!(wilders.next(87.77), Some(85.5065525725184));
        assert_eq!(wilders.next(87.29), Some(85.86324205801472));
    }
}
