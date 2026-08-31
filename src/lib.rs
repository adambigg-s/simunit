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
     use crate::unit::Unit;
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
          let m: value::Value<f32> = Meter.as_value();
          println!("{}", m.display_name());
     }
}
