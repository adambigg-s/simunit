use std::any;
use std::cmp;
use std::f64;
use std::fmt;
use std::ops;
use std::sync;

use rustc_hash as rh;

use crate::number_traits::AlmostInteger;
use crate::unit;

#[derive(Clone)]
pub struct UnitDimensionality
{
     dim: f64,
     _unit: sync::Arc<dyn unit::Unit>,
}

impl UnitDimensionality
{
     pub fn new_unit<U>(unit: U) -> Self
     where
          U: unit::Unit,
     {
          Self {
               dim: 1.0,
               _unit: sync::Arc::new(unit),
          }
     }

     pub fn new_inv_unit<U>(unit: U) -> Self
     where
          U: unit::Unit,
     {
          Self {
               dim: -1.0,
               _unit: sync::Arc::new(unit),
          }
     }

     pub fn with_exp<U>(unit: U, exp: f64) -> Self
     where
          U: unit::Unit,
     {
          Self {
               dim: exp,
               _unit: sync::Arc::new(unit),
          }
     }

     pub fn dim(&self) -> f64
     {
          self.dim
     }

     pub fn set_dim(&mut self, dim: f64)
     {
          self.dim = dim;
     }

     pub fn unit(&self) -> sync::Arc<dyn unit::Unit>
     {
          sync::Arc::clone(&self._unit)
     }

     pub fn set_unit(&mut self, _unit: sync::Arc<dyn unit::Unit>)
     {
          self._unit = _unit;
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

impl fmt::Display for UnitDimensionality
{
     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
     {
          if !self.dim.almost()
          {
               write!(fmt, "{}^{:.2}", self._unit.display_name(), self.dim)?;
               return Ok(());
          }

          if self.dim().round() == 0.0
          {
               return Ok(());
          }
          else if self.dim().round() == 1.0
          {
               write!(fmt, "{}", self._unit.display_name())?;
          }
          else
          {
               write!(fmt, "{}^{:.0}", self._unit.display_name(), self.dim.round())?;
          }
          Ok(())
     }
}

impl<U> From<U> for UnitDimensionality
where
     U: unit::Unit,
{
     fn from(value: U) -> Self
     {
          Self::new_unit(value)
     }
}

impl cmp::PartialEq for UnitDimensionality
{
     fn eq(&self, other: &Self) -> bool
     {
          self.dim == other.dim
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

     pub fn inner(&self) -> &rh::FxHashMap<any::TypeId, UnitDimensionality>
     {
          &self.inner
     }

     pub fn set_inner(&mut self, inner: rh::FxHashMap<any::TypeId, UnitDimensionality>)
     {
          self.inner = inner;
     }
}

impl From<UnitDimensionality> for UnitCache
{
     fn from(value: UnitDimensionality) -> Self
     {
          Self {
               inner: rh::FxHashMap::from_iter(vec![((*value._unit).type_id(), value)]),
          }
     }
}

impl cmp::PartialEq for UnitCache
{
     fn eq(&self, other: &Self) -> bool
     {
          if self.inner.len() != other.inner.len()
          {
               return false;
          }

          self.inner
               .iter()
               .all(|(unit, outer_dimension)| other.inner.get(unit) == Some(outer_dimension))
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
               .or_insert(UnitDimensionality::new_unit(rhs));
     }
}

impl<U> ops::DivAssign<U> for UnitCache
where
     U: unit::Unit + 'static,
{
     #[allow(clippy::suspicious_op_assign_impl)]
     fn div_assign(&mut self, rhs: U)
     {
          self.inner
               .entry(rhs.type_id())
               .and_modify(|entry| entry.dim -= 1.0)
               .or_insert(UnitDimensionality::new_inv_unit(rhs));
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

impl ops::DivAssign for UnitCache
{
     fn div_assign(&mut self, rhs: Self)
     {
          rhs.inner.into_iter().for_each(|(id, ud)| {
               self.inner.entry(id).and_modify(|entry| entry.dim -= ud.dim).or_insert(ud);
          });
     }
}
