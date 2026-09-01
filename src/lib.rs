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
     fn main_for_tests()
     {
          let m = Meter;
          let s = Second;
          let d = value::Value::new(3.0f32) * m.as_value();
          let t = d.clone() * s;
          let v = d.clone() / s;
          dbg!(d);
          dbg!(t);
          dbg!(v);
     }
}
