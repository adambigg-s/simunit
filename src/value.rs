#[cfg(debug_assertions)]
use std::error;
#[cfg(debug_assertions)]
use std::fmt;
use std::ops;

#[cfg(debug_assertions)]
use crate::number_traits;
use crate::unit;
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

     pub fn _units(&self) -> &uc::UnitCache
     {
          &self._units
     }

     pub fn _error(&self) -> Option<&ValueError>
     {
          self._error.as_ref()
     }
}

#[cfg(not(debug_assertions))]
impl<V> Value<V>
{
     pub fn new(value: V) -> Self
     {
          Self {
               value,
          }
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

impl<V> AsValue<V> for V
where
     V: Clone,
{
     fn as_value(&self) -> Value<V>
     {
          Value::new(self.clone())
     }
}

#[cfg(debug_assertions)]
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
impl<V> From<uc::UnitCache> for Value<V>
where
     V: number_traits::Number,
{
     fn from(value: uc::UnitCache) -> Self
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
impl<V> ops::BitXor<V> for Value<V>
where
     V: number_traits::Number + Copy,
{
     type Output = Self;

     fn bitxor(mut self, rhs: V) -> Self::Output
     {
          self._units = self._units ^ rhs.to_f64();
          self.value = self.value.pow(rhs);
          self
     }
}

#[cfg(debug_assertions)]
impl<V> ops::BitXor<Value<V>> for Value<V>
where
     V: number_traits::Number + Copy,
{
     type Output = Self;

     fn bitxor(mut self, rhs: Value<V>) -> Self::Output
     {
          if !rhs._units.dimensionless()
          {
               self._error = Some(ValueError::UnitExponentiationError {
                    exp: rhs._units.clone(),
               })
          }

          self = self ^ rhs.value;
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
impl<V> ops::MulAssign for Value<V>
where
     V: ops::MulAssign + ops::Mul<Output = V> + Clone,
{
     fn mul_assign(&mut self, rhs: Self)
     {
          *self = self.clone() * rhs;
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
impl<V> ops::Div for Value<V>
where
     V: ops::Div<Output = V>,
{
     type Output = Value<V>;

     fn div(mut self, rhs: Self) -> Self::Output
     {
          self._units /= rhs._units;
          self.value = self.value / rhs.value;
          self
     }
}

#[cfg(debug_assertions)]
impl<V> ops::DivAssign for Value<V>
where
     V: ops::DivAssign + ops::Div<Output = V> + Clone,
{
     fn div_assign(&mut self, rhs: Self)
     {
          *self = self.clone() / rhs;
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
impl<V> ops::AddAssign for Value<V>
where
     V: ops::AddAssign + ops::Add<Output = V> + Clone,
{
     fn add_assign(&mut self, rhs: Self)
     {
          *self = self.clone() + rhs;
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

#[cfg(debug_assertions)]
impl<V> ops::SubAssign for Value<V>
where
     V: ops::SubAssign + ops::Sub<Output = V> + Clone,
{
     fn sub_assign(&mut self, rhs: Self)
     {
          *self = self.clone() - rhs;
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::BitXor<f64> for Value<V>
{
     type Output = Self;

     fn bitxor(self, _: f64) -> Self::Output
     {
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::Mul for Value<V>
where
     V: ops::Mul<Output = V>,
{
     type Output = Value<V>;

     fn mul(mut self, rhs: Self) -> Self::Output
     {
          self.value = self.value * rhs.value;
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::MulAssign for Value<V>
where
     V: ops::MulAssign + ops::Mul<Output = V> + Clone,
{
     fn mul_assign(&mut self, rhs: Self)
     {
          *self = self.clone() * rhs;
     }
}

#[cfg(not(debug_assertions))]
impl<V, U> ops::Mul<U> for Value<V>
where
     U: unit::Unit + 'static,
{
     type Output = Value<V>;

     fn mul(self, _: U) -> Self::Output
     {
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::Div for Value<V>
where
     V: ops::Div<Output = V>,
{
     type Output = Value<V>;

     fn div(mut self, rhs: Self) -> Self::Output
     {
          self.value = self.value / rhs.value;
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::DivAssign for Value<V>
where
     V: ops::DivAssign + ops::Div<Output = V> + Clone,
{
     fn div_assign(&mut self, rhs: Self)
     {
          *self = self.clone() / rhs;
     }
}

#[cfg(not(debug_assertions))]
impl<V, U> ops::Div<U> for Value<V>
where
     U: unit::Unit + 'static,
{
     type Output = Value<V>;

     fn div(self, _: U) -> Self::Output
     {
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::Add for Value<V>
where
     V: ops::Add<Output = V>,
{
     type Output = Value<V>;

     fn add(mut self, rhs: Self) -> Self::Output
     {
          self.value = self.value + rhs.value;
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::AddAssign for Value<V>
where
     V: ops::AddAssign + ops::Add<Output = V> + Clone,
{
     fn add_assign(&mut self, rhs: Self)
     {
          *self = self.clone() + rhs;
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::Sub for Value<V>
where
     V: ops::Sub<Output = V>,
{
     type Output = Value<V>;

     fn sub(mut self, rhs: Self) -> Self::Output
     {
          self.value = self.value - rhs.value;
          self
     }
}

#[cfg(not(debug_assertions))]
impl<V> ops::SubAssign for Value<V>
where
     V: ops::SubAssign + ops::Sub<Output = V> + Clone,
{
     fn sub_assign(&mut self, rhs: Self)
     {
          *self = self.clone() - rhs;
     }
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub enum ValueError
{
     UnitAdditionMismatch
     {
          rhs: uc::UnitCache,
          lhs: uc::UnitCache,
     },
     UnitSubtractionMismatch
     {
          rhs: uc::UnitCache,
          lhs: uc::UnitCache,
     },
     UnitExponentiationError
     {
          exp: uc::UnitCache,
     },
}

#[cfg(debug_assertions)]
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
               | ValueError::UnitExponentiationError {
                    exp,
               } =>
               {
                    write!(
                         fmt,
                         "Unit exponent mismatch\nEXP: {:?}\nExponent must be dimensionless for a physically signifigant quantity",
                         exp
                    )?
               }
          }

          Ok(())
     }
}

#[cfg(debug_assertions)]
impl error::Error for ValueError {}

#[cfg(test)]
#[cfg(debug_assertions)]
mod ttt_value
{
     use super::*;

     struct TestingUnit;
     impl unit::Unit for TestingUnit {}

     #[test]
     fn value_addition()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let add = v0 + v1;
          assert!(add.value == 2);
          assert!(add._error.is_none());
          assert!(add._units.check_homogenous(vec![uc::UnitDimensionality::new_unit(TestingUnit)]));
     }

     #[test]
     fn value_addition_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut add: Value<i32> = <i32 as number_traits::AddIdentity>::zero().as_value() * TestingUnit;
          add += v0;
          add += v1;
          assert!(add.value == 2);
          assert!(add._error.is_none());
          assert!(add._units.check_homogenous(vec![uc::UnitDimensionality::new_unit(TestingUnit)]));
     }

     #[test]
     fn value_subtraction()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let sub = v0 - v1;
          assert!(sub.value == 0);
          assert!(sub._error.is_none());
          assert!(sub._units.check_homogenous(vec![uc::UnitDimensionality::new_unit(TestingUnit)]));
     }

     #[test]
     fn value_subtraction_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut add: Value<i32> = <i32 as number_traits::AddIdentity>::zero().as_value() * TestingUnit;
          add -= v0;
          add -= v1;
          assert!(add.value == -2);
          assert!(add._error.is_none());
          assert!(add._units.check_homogenous(vec![uc::UnitDimensionality::new_unit(TestingUnit)]));
     }

     #[test]
     fn value_multiplication()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mul = v0 * v1;
          assert!(mul.value == 1);
          assert!(mul._error.is_none());
          assert!(mul._units.check_homogenous(vec![
               uc::UnitDimensionality::new_unit(TestingUnit),
               uc::UnitDimensionality::new_unit(TestingUnit),
          ]));
     }

     #[test]
     fn value_multiplication_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut mul = <i32 as number_traits::MulIdentity>::one().as_value();
          mul *= v0;
          mul *= v1;
          assert!(mul.value == 1);
          assert!(mul._error.is_none());
          assert!(mul._units.check_homogenous(vec![
               uc::UnitDimensionality::new_unit(TestingUnit),
               uc::UnitDimensionality::new_unit(TestingUnit),
          ]));
     }

     #[test]
     fn value_division()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let div = v0 / v1;
          assert!(div.value == 1);
          assert!(div._error.is_none());
          assert!(div._units.check_homogenous(vec![]));
     }

     #[test]
     fn value_division_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut mul = <i32 as number_traits::MulIdentity>::one().as_value();
          mul /= v0;
          mul /= v1;
          assert!(mul.value == 1);
          assert!(mul._error.is_none());
          assert!(mul._units.check_homogenous(vec![
               uc::UnitDimensionality::new_inv_unit(TestingUnit),
               uc::UnitDimensionality::new_inv_unit(TestingUnit),
          ]));
     }

     #[test]
     fn value_exponentiation()
     {
          let v = (2.0).as_value() * TestingUnit;
          let exp = v ^ 3.0;
          assert!(exp.value == 8.0);
          assert!(exp._error.is_none());
          assert!(exp._units.check_homogenous(vec![
               uc::UnitDimensionality::new_inv_unit(TestingUnit),
               uc::UnitDimensionality::new_inv_unit(TestingUnit),
               uc::UnitDimensionality::new_inv_unit(TestingUnit),
          ]))
     }
}

#[cfg(test)]
#[cfg(not(debug_assertions))]
mod ttt_value
{
     use super::*;
     use crate::number_traits;

     struct TestingUnit;
     impl unit::Unit for TestingUnit {}

     #[test]
     fn value_addition()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let add = v0 + v1;
          assert!(add.value == 2);
     }

     #[test]
     fn value_addition_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut add: Value<i32> = <i32 as number_traits::AddIdentity>::zero().as_value() * TestingUnit;
          add += v0;
          add += v1;
          assert!(add.value == 2);
     }

     #[test]
     fn value_subtraction()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let sub = v0 - v1;
          assert!(sub.value == 0);
     }

     #[test]
     fn value_subtraction_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut add: Value<i32> = <i32 as number_traits::AddIdentity>::zero().as_value() * TestingUnit;
          add -= v0;
          add -= v1;
          assert!(add.value == -2);
     }

     #[test]
     fn value_multiplication()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mul = v0 * v1;
          assert!(mul.value == 1);
     }

     #[test]
     fn value_multiplication_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut mul = <i32 as number_traits::MulIdentity>::one().as_value();
          mul *= v0;
          mul *= v1;
          assert!(mul.value == 1);
     }

     #[test]
     fn value_division()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let div = v0 / v1;
          assert!(div.value == 1);
     }

     #[test]
     fn value_division_assign()
     {
          let v0 = 1.as_value() * TestingUnit;
          let v1 = 1.as_value() * TestingUnit;
          let mut mul = <i32 as number_traits::MulIdentity>::one().as_value();
          mul /= v0;
          mul /= v1;
          assert!(mul.value == 1);
     }
}
