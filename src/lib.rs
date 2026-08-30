use std::any::Any;
use std::any::{self};
use std::ops;

pub use rustc_hash as rh;

pub struct SimUnit<V>
{
     pub value: V,
     #[cfg(debug_assertions)]
     _units: UnitCache,
}

#[cfg(debug_assertions)]
impl<V> SimUnit<V>
{
     pub fn new(value: V) -> Self
     {
          Self {
               value,
               _units: UnitCache::new(),
          }
     }
}

#[cfg(not(debug_assertions))]
impl<V> SimUnit<V>
{
     pub fn new(value: V) -> Self
     {
          Self {
               value,
               // _units: UnitCache::new(),
          }
     }
}

#[cfg(not(debug_assertions))]
impl<V, U> ops::Mul<U> for SimUnit<V>
where
     U: Unit + 'static,
{
     type Output = SimUnit<V>;

     fn mul(mut self, rhs: U) -> Self::Output
     {
          // self._units *= rhs;
          self
     }
}

#[cfg(debug_assertions)]
impl<V, U> ops::Mul<U> for SimUnit<V>
where
     U: Unit + 'static,
{
     type Output = SimUnit<V>;

     fn mul(mut self, rhs: U) -> Self::Output
     {
          self._units *= rhs;
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::BitXor<f64> for SimUnit<V>
{
     type Output = Self;

     fn bitxor(self, rhs: f64) -> Self::Output
     {
          Self {
               value: self.value,
               // _units: self._units ^ rhs,
          }
     }
}

#[cfg(debug_assertions)]
impl<V> ops::BitXor<f64> for SimUnit<V>
{
     type Output = Self;

     fn bitxor(self, rhs: f64) -> Self::Output
     {
          Self {
               value: self.value,
               _units: self._units ^ rhs,
          }
     }
}

pub trait Unit
{
     fn display_name(&self) -> &'static str
     {
          ""
     }
}

pub struct UnitDimensionality
{
     dim: f64,
     _unit: Box<dyn Unit>,
}

#[derive(Default)]
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
     U: Unit + 'static,
{
     fn mul_assign(&mut self, rhs: U)
     {
          let t_id = rhs.type_id();
          // let x = self.inner.get_mut(&t_id).get_or_insert_default();
          // x.dim += rhs;
     }
}

#[cfg(test)]
mod tests
{
     use super::*;

     pub struct Meter;

     impl Unit for Meter {}

     #[test]
     fn main_test()
     {
          let su = SimUnit::new(10) * Meter;
          assert!(su.value == 10);
     }
}
