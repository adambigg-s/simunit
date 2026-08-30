use std::ops;

use crate::unit;
use crate::unit_cache as uc;

#[derive(Debug, Default, Clone)]
pub struct Value<V>
{
     pub value: V,
     #[cfg(debug_assertions)]
     _units: uc::UnitCache,
}

#[cfg(debug_assertions)]
impl<V> Value<V>
{
     pub fn new(value: V) -> Self
     {
          Self {
               value,
               _units: uc::UnitCache::new(),
          }
     }
}

#[cfg(debug_assertions)]
impl<V, U> ops::Mul<U> for Value<V>
where
     U: unit::Unit + 'static,
{
     type Output = Value<V>;

     fn mul(mut self, rhs: U) -> Self::Output
     {
          self._units *= rhs;
          self
     }
}

#[cfg(debug_assertions)]
impl<V> ops::BitXor<f64> for Value<V>
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

#[cfg(not(debug_assertions))]
impl<V> Value<V>
{
     pub fn new(value: V) -> Self
     {
          Self {
               Value,
          }
     }
}

#[cfg(not(debug_assertions))]
impl<V, U> ops::Mul<U> for Value<V>
where
     U: unit::Unit + 'static,
{
     type Output = Value<V>;

     fn mul(mut self, rhs: U) -> Self::Output
     {
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::BitXor<f64> for Value<V>
{
     type Output = Self;

     fn bitxor(self, rhs: f64) -> Self::Output
     {
          Self {
               value: self.value,
          }
     }
}
