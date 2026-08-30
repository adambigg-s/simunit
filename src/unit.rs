use std::any;

pub trait Unit
where
     Self: any::Any,
{
     fn display_name(&self) -> &'static str
     {
          any::type_name::<Self>()
     }
}
