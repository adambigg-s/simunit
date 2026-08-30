use std::any;
use std::f64;
use std::fmt;
use std::ops;
use std::rc;

use rustc_hash as rh;

use crate::unit;

#[derive(Clone)]
pub struct UnitDimensionality
{
     dim: f64,
     _unit: rc::Rc<dyn unit::Unit>,
}

impl UnitDimensionality
{
     pub fn new<U>(unit: U) -> Self
     where
          U: unit::Unit,
     {
          Self {
               dim: 1.0,
               _unit: rc::Rc::new(unit),
          }
     }

     pub fn with_exp<U>(unit: U, exp: f64) -> Self
     where
          U: unit::Unit,
     {
          Self {
               dim: exp,
               _unit: rc::Rc::new(unit),
          }
     }
}

impl fmt::Debug for UnitDimensionality
{
     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
     {
          fmt.debug_struct("UnitDimensionality")
               .field("dim", &self.dim)
               .field("_unit", &self._unit.display_name())
               .finish()
     }
}

#[derive(Debug, Default, Clone)]
pub struct UnitCache
{
     inner: rh::FxHashMap<any::TypeId, UnitDimensionality>,
}

impl UnitCache
{
     pub fn new() -> Self
     {
          Self::default()
     }

     pub fn unit_dimensionality(&self) -> Vec<UnitDimensionality>
     {
          self.inner.values().cloned().collect()
     }
}

impl ops::BitXor<f64> for UnitCache
{
     type Output = Self;

     fn bitxor(mut self, rhs: f64) -> Self::Output
     {
          self.inner.values_mut().for_each(|ud| ud.dim *= rhs);
          self
     }
}

impl<U> ops::MulAssign<U> for UnitCache
where
     U: unit::Unit + 'static,
{
     #[allow(clippy::suspicious_op_assign_impl)]
     fn mul_assign(&mut self, rhs: U)
     {
          self.inner
               .entry(rhs.type_id())
               .and_modify(|entry| entry.dim += 1.0)
               .or_insert(UnitDimensionality::new(rhs));
     }
}

impl ops::MulAssign for UnitCache
{
     fn mul_assign(&mut self, rhs: Self)
     {
          rhs.inner.into_iter().for_each(|(id, ud)| {
               self.inner.entry(id).and_modify(|entry| entry.dim += ud.dim).or_insert(ud);
          });
     }
}
