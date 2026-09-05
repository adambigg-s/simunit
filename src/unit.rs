use std::any;

pub trait Unit
where
     Self: any::Any,
{
     fn display_name(&self) -> &'static str
     {
          any::type_name::<Self>().split("::").last().unwrap()
     }
}

#[cfg(test)]
#[cfg(not(debug_assertions))]
mod ttt_unit
{
     use super::*;
     use crate::unit;

     struct TestingUnit;
     impl unit::Unit for TestingUnit {}

     #[test]
     fn unit_default_name()
     {
          assert!(TestingUnit.display_name() == "TestingUnit");
     }

     #[test]
     fn unit_named_name()
     {
          pub const EXT: &str = "ext";
          struct DefaultUnitExt(TestingUnit);
          impl unit::Unit for DefaultUnitExt
          {
               fn display_name(&self) -> &'static str
               {
                    EXT
               }
          }

          assert!(DefaultUnitExt(TestingUnit).display_name() == EXT);
     }
}

#[cfg(test)]
#[cfg(debug_assertions)]
mod ttt_unit
{
     use crate::unit;
     use crate::unit_cache;
     use crate::value::AsValue;

     struct TestingUnitWithFields
     {
          _x: i32,
          _y: f32,
     }

     impl unit::Unit for TestingUnitWithFields {}

     #[test]
     fn unit_with_fields_attach()
     {
          let u = TestingUnitWithFields {
               _x: 0,
               _y: 0.0,
          };
          let v = 1.as_value() * u;
          assert!(v.value == 1);
          assert!(v._units().check_homogenous(vec![unit_cache::UnitDimensionality::new_unit(
               TestingUnitWithFields {
                    _x: i32::MAX,
                    _y: f32::MAX
               }
          )]))
     }
}
