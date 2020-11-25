[![Build Status](https://travis-ci.com/immortalinfidel/wilders-rs.svg?branch=master)](https://travis-ci.com/immortalinfidel/wilders-rs)

# Wilders Smoothing  

```
use ta_common::traits::Indicator;
use wilders_rs::Wilders;

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
```