pub use implementation::*;

#[rustfmt::skip]
#[allow(clippy::unnecessary_cast)]
pub mod implementation {
     pub trait AddIdentity {
          fn zero() -> Self;
     }

     impl AddIdentity for u8   { fn zero() -> Self { 0 as u8   } }
     impl AddIdentity for u16  { fn zero() -> Self { 0 as u16  } }
     impl AddIdentity for u32  { fn zero() -> Self { 0 as u32  } }
     impl AddIdentity for u64  { fn zero() -> Self { 0 as u64  } }
     impl AddIdentity for u128 { fn zero() -> Self { 0 as u128 } }

     impl AddIdentity for i8   { fn zero() -> Self { 0 as i8   } }
     impl AddIdentity for i16  { fn zero() -> Self { 0 as i16  } }
     impl AddIdentity for i32  { fn zero() -> Self { 0 as i32  } }
     impl AddIdentity for i64  { fn zero() -> Self { 0 as i64  } }
     impl AddIdentity for i128 { fn zero() -> Self { 0 as i128 } }

     impl AddIdentity for f16  { fn zero() -> Self { 0 as f16  } }
     impl AddIdentity for f32  { fn zero() -> Self { 0 as f32  } }
     impl AddIdentity for f64  { fn zero() -> Self { 0 as f64  } }
     impl AddIdentity for f128 { fn zero() -> Self { 0 as f128 } }

     pub trait MulIdentity
     {
          fn one() -> Self;
     }

     impl MulIdentity for u8   { fn one() -> Self { 1 as u8   } }
     impl MulIdentity for u16  { fn one() -> Self { 1 as u16  } }
     impl MulIdentity for u32  { fn one() -> Self { 1 as u32  } }
     impl MulIdentity for u64  { fn one() -> Self { 1 as u64  } }
     impl MulIdentity for u128 { fn one() -> Self { 1 as u128 } }

     impl MulIdentity for i8   { fn one() -> Self { 1 as i8   } }
     impl MulIdentity for i16  { fn one() -> Self { 1 as i16  } }
     impl MulIdentity for i32  { fn one() -> Self { 1 as i32  } }
     impl MulIdentity for i64  { fn one() -> Self { 1 as i64  } }
     impl MulIdentity for i128 { fn one() -> Self { 1 as i128 } }

     impl MulIdentity for f16  { fn one() -> Self { 1 as f16  } }
     impl MulIdentity for f32  { fn one() -> Self { 1 as f32  } }
     impl MulIdentity for f64  { fn one() -> Self { 1 as f64  } }
     impl MulIdentity for f128 { fn one() -> Self { 1 as f128 } }

     pub trait Number
     where
          Self: AddIdentity + MulIdentity,
     {
          fn to_f16(self) -> f16;

          fn to_f32(self) -> f32;

          fn to_f64(self) -> f64;

          fn to_f128(self) -> f128;
     }

     impl Number for u8   { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for u16  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for u32  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for u64  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for u128 { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }

     impl Number for i8   { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for i16  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for i32  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for i64  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for i128 { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }

     impl Number for f16  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for f32  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for f64  { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }
     impl Number for f128 { fn to_f16(self) -> f16 { self as f16 } fn to_f32(self) -> f32 { self as f32 } fn to_f64(self) -> f64 { self as f64 } fn to_f128(self) -> f128 { self as f128 } }

     pub trait AlmostInteger
     where
          Self: Number,
     {
          fn almost(&self) -> bool;
     }

     impl AlmostInteger for f16  { fn almost(&self) -> bool { (self - self.round() as u16 as f16) < f16::EPSILON    } }
     impl AlmostInteger for f32  { fn almost(&self) -> bool { (self - self.round() as u32 as f32) < f32::EPSILON    } }
     impl AlmostInteger for f64  { fn almost(&self) -> bool { (self - self.round() as u64 as f64) < f64::EPSILON    } }
     impl AlmostInteger for f128 { fn almost(&self) -> bool { (self - self.round() as u128 as f128) < f128::EPSILON } }
}

#[cfg(test)]
mod ttt_number_traits
{}
