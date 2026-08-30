use std::any;
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
     fn mul_assign(&mut self, rhs: U)
     {
          let t_id = rhs.type_id();
          // self.inner.entry(t_id).
          // let x = self.inner.get_mut(&t_id).get_or_insert_default();
          // x.dim += rhs;
     }
}
