use crate::unit_cache;

pub trait UnitRedux
{
     fn reduce(&mut self, registry: ReduxRule);
}

pub struct ReduxRule
{
     pattern: unit_cache::UnitCache,
     replacement: unit_cache::UnitDimensionality,
}

impl ReduxRule
{
     pub fn new(
          pattern: Vec<unit_cache::UnitDimensionality>,
          replacement: unit_cache::UnitDimensionality,
     ) -> Self
     {
          Self {
               pattern: unit_cache::UnitCache::from(pattern),
               replacement,
          }
     }

     pub fn pattern(&self) -> &unit_cache::UnitCache
     {
          &self.pattern
     }

     pub fn replacement(&self) -> &unit_cache::UnitDimensionality
     {
          &self.replacement
     }
}
