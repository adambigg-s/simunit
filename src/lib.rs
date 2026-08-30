use std::{any::{self, Any}, ops};

pub use rustc_hash as rh;

pub struct SimUnit<T> {
    pub value: T,
    units: UnitCache,
}

impl<T> SimUnit<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            units: UnitCache::new(),
        }
    }
}

impl<T> ops::Mul<&dyn Unit> for SimUnit<T> {
    type Output = SimUnit<T>;

    fn mul(mut self, rhs: &dyn Unit) -> Self::Output {
        self.units *= rhs;
        self
    }
}

impl<T> ops::BitXor<f64> for SimUnit<T> {
    type Output = Self;

    fn bitxor(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value,
            units: self.units ^ rhs,
        }
    }
}

pub trait Unit {
    fn display_name(&self) -> &'static str {
        ""
    }
}

#[derive(Default)]
pub struct UnitCache {
    inner: rh::FxHashMap<any::TypeId, (Box<dyn Unit>, f64)>,
}

impl UnitCache {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ops::BitXor<f64> for UnitCache {
    type Output = Self;

    fn bitxor(mut self, rhs: f64) -> Self::Output {
        self.inner.values_mut().for_each(|(_, val)| *val *= rhs);
        self
    }
}

impl ops::MulAssign<&dyn Unit> for UnitCache {
    fn mul_assign(&mut self, rhs: &dyn Unit) {
        let key = rhs.type_id();
        let entry = self.inner.entry(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Meter;

    impl Unit for Meter {}

    #[test]
    fn main_test() {
        let su = SimUnit::new(10) * &Meter;
    }
}
