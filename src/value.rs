use std::error;
use std::fmt;
use std::ops;

use crate::number_traits;
use crate::unit;
use crate::unit_cache;
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
     #[cfg(debug_assertions)]
     _error: Option<ValueError>,
}

#[cfg(debug_assertions)]
impl<V> Value<V>
{
     pub fn new(value: V) -> Self
     {
          Self {
               value,
               _units: uc::UnitCache::new(),
               _error: None,
          }
     }

     pub fn units(&self) -> Vec<uc::UnitDimensionality>
     {
          self._units.unit_dimensionality()
     }
}

impl<V> AsValue<V> for Value<V>
where
     V: Clone,
{
     fn as_value(&self) -> Value<V>
     {
          (*self).clone()
     }
}

impl<V> AsValue<V> for &Value<V>
where
     V: Copy,
{
     fn as_value(&self) -> Value<V>
     {
          (*self).clone()
     }
}

impl<V> fmt::Display for Value<V>
where
     V: fmt::Display,
{
     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
     {
          if let Some(error) = &self._error
          {
               write!(fmt, "{}", error)?;
               return Err(fmt::Error);
          }

          write!(fmt, "{}", self.value)?;
          write!(fmt, " [")?;
          let mut units = self._units.unit_dimensionality();
          units.sort_unstable_by(|a, b| a.unit().display_name().cmp(b.unit().display_name()));
          let mut units_iter = units.into_iter().peekable();
          while let Some(unit) = units_iter.next()
          {
               if units_iter.peek().is_some()
               {
                    write!(fmt, "{} * ", unit)?;
               }
               else
               {
                    write!(fmt, "{}", unit)?;
               }
          }
          write!(fmt, "]")?;
          Ok(())
     }
}

#[cfg(debug_assertions)]
impl<V> From<unit_cache::UnitCache> for Value<V>
where
     V: number_traits::Number,
{
     fn from(value: unit_cache::UnitCache) -> Self
     {
          Self {
               value: V::one(),
               _units: value,
               _error: None,
          }
     }
}

#[cfg(debug_assertions)]
impl<V> From<V> for Value<V>
where
     V: number_traits::Number,
{
     fn from(value: V) -> Self
     {
          Self::new(value)
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
               _error: None,
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
impl<V> ops::Add for Value<V>
where
     V: ops::Add<Output = V>,
{
     type Output = Value<V>;

     fn add(mut self, rhs: Self) -> Self::Output
     {
          if self._units != rhs._units
          {
               self._error = Some(ValueError::UnitAdditionMismatch {
                    rhs: rhs._units.clone(),
                    lhs: self._units.clone(),
               })
          }

          self.value = self.value + rhs.value;
          self
     }
}

#[cfg(debug_assertions)]
impl<V> ops::Sub for Value<V>
where
     V: ops::Sub<Output = V>,
{
     type Output = Value<V>;

     fn sub(mut self, rhs: Self) -> Self::Output
     {
          if self._units != rhs._units
          {
               self._error = Some(ValueError::UnitAdditionMismatch {
                    rhs: rhs._units.clone(),
                    lhs: self._units.clone(),
               })
          }

          self.value = self.value - rhs.value;
          self
     }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueError
{
     UnitAdditionMismatch
     {
          rhs: unit_cache::UnitCache,
          lhs: unit_cache::UnitCache,
     },
     UnitSubtractionMismatch
     {
          rhs: unit_cache::UnitCache,
          lhs: unit_cache::UnitCache,
     },
}

impl fmt::Display for ValueError
{
     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
     {
          match self
          {
               | ValueError::UnitAdditionMismatch {
                    rhs,
                    lhs,
               } => writeln!(fmt, "Unit addition mismatch between\nRHS: {:?}\nLHS: {:?}", rhs, lhs)?,
               | ValueError::UnitSubtractionMismatch {
                    rhs,
                    lhs,
               } => writeln!(fmt, "Unit subtraction mismatch between\nRHS: {:?}\nLHS: {:?}", rhs, lhs)?,
          }
          Ok(())
     }
}

impl error::Error for ValueError {}

#[cfg(test)]
mod ttt_value
{
     use super::*;

     #[derive(Clone, Copy)]
     struct DefaultUnit;
     impl unit::Unit for DefaultUnit {}

     #[test]
     fn value_addition()
     {
          let v0 = Value::new(1) * DefaultUnit;
          let v1 = Value::new(1) * DefaultUnit;
          let add = v0 + v1;
          assert!(add.value == 2);
          assert!(add._error.is_none());
     }

     #[test]
     fn value_subtraction()
     {
          let v0 = Value::new(1) * DefaultUnit;
          let v1 = Value::new(1) * DefaultUnit;
          let add = v0 - v1;
          assert!(add.value == 0);
          assert!(add._error.is_none());
     }

     #[test]
     fn value_multiplication()
     {
          let v0 = Value::new(1) * DefaultUnit;
          let v1 = Value::new(1) * DefaultUnit;
          let add = v0 + v1;
          assert!(add.value == 2);
          assert!(add._error.is_none());
     }

     #[test]
     fn value_division()
     {
          let v0 = Value::new(1) * DefaultUnit;
          let v1 = Value::new(1) * DefaultUnit;
          let add = v0 - v1;
          assert!(add.value == 0);
          assert!(add._error.is_none());
     }
}
