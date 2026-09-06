#![feature(f16)]
#![feature(f128)]

pub mod builtin;
pub mod conversion;
pub mod number_traits;
pub mod redux;
pub mod unit;
pub mod unit_cache;
pub mod value;

pub use unit::Unit;
pub use value::AsValue;
pub use value::Value;

#[cfg(test)]
mod practical_test
{
     use crate::AsValue;
     use crate::unit;
     use crate::unit_cache;

     unit!(Kilogram => "kg");
     unit!(Meter => "m");
     unit!(Second => "s");

     const G: f32 = 6.674e-11;

     fn gravitational_attraction(m0: f32, m1: f32, d: f32) -> f32
     {
          let g = G.as_value() * Meter * Meter * Meter / Kilogram / Second / Second;

          let m0 = m0.as_value() * Kilogram;
          let m1 = m1.as_value() * Kilogram;
          let d = d.as_value() * Meter;

          let f = g * m0 * m1 / (d ^ 2.0);

          assert!(f._units().check_homogenous(vec![
               unit_cache::UnitDimensionality::new_unit(Kilogram),
               unit_cache::UnitDimensionality::new_unit(Meter),
               unit_cache::UnitDimensionality::new_inv_unit(Second),
               unit_cache::UnitDimensionality::new_inv_unit(Second),
          ]));

          f.value
     }

     #[test]
     fn gravity_unit_test()
     {
          _ = gravitational_attraction(10.0, 10.0, 10.0);
     }
}
