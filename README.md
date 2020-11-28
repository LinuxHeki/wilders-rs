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
```