use std::ops;

use crate::number_traits;
use crate::unit;
use crate::unit_cache;
#[cfg(debug_assertions)]
use crate::unit_cache as uc;

pub trait AsValue<V>
{
     fn as_value(&self) -> Value<V>;
}

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

     pub fn units(&self) -> Vec<uc::UnitDimensionality>
     {
          self._units.unit_dimensionality()
     }
}

impl<V> From<unit_cache::UnitCache> for Value<V>
where
     V: number_traits::Number,
{
     fn from(value: unit_cache::UnitCache) -> Self
     {
          Self {
               value: V::one(),
               _units: value,
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
impl<V> ops::Mul for Value<V>
where
     V: ops::Mul<Output = V>,
{
     type Output = Value<V>;

     fn mul(mut self, rhs: Self) -> Self::Output
     {
          self._units *= rhs._units;
          self.value = self.value * rhs.value;
          self
     }
}

#[cfg(debug_assertions)]
impl<V, U> ops::Div<U> for Value<V>
where
     U: unit::Unit + 'static,
{
     type Output = Value<V>;

     fn div(mut self, rhs: U) -> Self::Output
     {
          self._units /= rhs;
          self
     }
}

#[cfg(debug_assertions)]
impl<V> ops::Div for Value<V>
where
     V: ops::Div<Output = V>,
{
     type Output = Value<V>;

     fn div(mut self, rhs: Self) -> Self::Output
     {
          self._units *= rhs._units;
          self.value = self.value / rhs.value;
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
