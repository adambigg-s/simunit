use std::any;

use crate::unit_cache;
use crate::value;

pub trait Unit
where
     Self: any::Any,
{
     fn display_name(&self) -> &'static str
     {
          any::type_name::<Self>()
     }
}

impl<V, U> value::AsValue<V> for U
where
     U: Unit + Copy,
     V: Default,
{
     fn as_value(&self) -> value::Value<V>
     {
          value::Value::from(unit_cache::UnitCache::from(unit_cache::UnitDimensionality::from(*self)))
     }
}
