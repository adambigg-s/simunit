#![feature(f16)]
#![feature(f128)]

pub mod number_traits;
pub mod unit;
pub mod unit_cache;
pub mod value;

#[cfg(test)]
mod tests
{
     use crate::unit;
     use crate::value;
     use crate::value::AsValue;

     #[derive(Clone, Copy)]
     pub struct Meter;

     impl unit::Unit for Meter {}

     #[derive(Clone, Copy)]
     pub struct Second;

     impl unit::Unit for Second {}

     #[test]
     fn main_test()
     {
          let su = value::Value::new(10) * (Meter.as_value() ^ 2.0);
          let xu = su.clone() * Second;
          // let tu = 10 * Meter;
          assert!(xu.value == 10);
          assert!(su.value == 10);

          let zz = su * xu;
          assert!(zz.value == 100);
          let dims = zz.units();
          assert!(dims.len() == 2);
          dbg!(dims);
     }
}

