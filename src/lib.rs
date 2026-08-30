pub mod unit;
pub mod unit_cache;
pub mod value;

#[cfg(test)]
mod tests
{
     use std::sync::Arc;

use crate::unit;
     use crate::value;

     pub struct Meter;

     impl unit::Unit for Meter {}

     pub struct Second;

     impl unit::Unit for Second {}

     #[test]
     fn main_test()
     {
          let su = value::Value::new(10) * Meter;
          let xu = su.clone() * Second;
          // let tu = 10 * Meter;
          assert!(xu.value == 10);
          assert!(su.value == 10);

          let zz = su * xu;
          assert!(zz.value == 100);
          let dims = zz.units();
          assert!(dims.len() == 2);
    }
}
