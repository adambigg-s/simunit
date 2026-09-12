#![allow(ambiguous_glob_reexports)]

pub use general::*;
pub use imperial::*;
pub use si::*;

#[macro_export]
macro_rules! unit {
     ($type:ident => $name:expr) => {
          #[derive(Debug, Default, Clone, Copy)]
          pub struct $type;
          impl $crate::unit::Unit for $type
          {
               fn display_name(&self) -> &'static str
               {
                    $name
               }
          }
     };
}
// unit!( => "");

pub mod general
{
     pub use dimensionless::*;

     pub mod dimensionless
     {
          unit!(Radian => "rad");
          unit!(Degree => "deg");
          /* ... */
     }
}

pub mod si
{
     pub use base::*;

     pub mod base
     {
          unit!(Meter => "m");
          unit!(Kilogram => "kg");
          unit!(Second => "s");
          unit!(Amp => "A");
          unit!(Kelvin => "K");
          unit!(Mole => "mol");
          unit!(Candela => "cd");
          /* ... */
     }

     pub mod derived
     {
          unit!(Newton => "N");
          unit!(Hertz => "Hz");
          unit!(Pascal => "Pa");
          unit!(Joule => "J");
          unit!(Watt => "W");
          unit!(Coulomb => "C");
          unit!(Volt => "V");
          unit!(Ohm => "Ohm");
          /* ... */
     }
}

pub mod imperial
{

     pub use base::*;
     pub use derived::*;

     pub mod base
     {
          unit!(PoundMass => "lbm");
          unit!(Yard => "yd");
          unit!(Second => "s");
          unit!(Gallon => "gal");
          /* ... */
     }

     pub mod derived
     {
          unit!(PoundForce => "lbf");
          unit!(Grain => "gr");
          unit!(Ounce => "oz");
          unit!(Stone => "st");
          unit!(Slug => "slug");
          unit!(Foot => "ft");
          unit!(Mile => "mi");
          unit!(Quart => "qt");
          unit!(FluidOz => "fl oz");
          /* ... */
     }
}
