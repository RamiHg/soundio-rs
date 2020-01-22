//! This is a simple crate that defines one trait for audio samples in various formats, and implements it
//! for the common sample formats. Because there is no native `u24` or `i24` I added one via the newtype
//! pattern using `u32` and `i32`.

use std;

/// The Sample trait defines functions to convert between the various sample formats. The full range
/// of the integer sample formats is always used, so `0u16.to_i8()` is -128. Converting between 
/// signed and unsigned of the same size is lossless, as is increasing the bit depth.
///
/// The range for floating point samples is -1.0 to 1.0 inclusive.
pub trait Sample {
	/// Convert from a u8 sample (0 - 0xFF) to this sample type.
	fn from_u8(v: u8) -> Self;
	/// Convert from a u16 sample (0 - 0xFFFF) to this sample type.
	fn from_u16(v: u16) -> Self;
	/// Convert from a u24 sample (0 - 0xFFFFFF) to this sample type.
	fn from_u24(v: u24) -> Self;
	/// Convert from a u32 sample (0 - 0xFFFFFFFF) to this sample type.
	fn from_u32(v: u32) -> Self;

	/// Convert from an i8 sample (-0x80 - 0x7F) to this sample type.
	fn from_i8(v: i8) -> Self;
	/// Convert from an i16 sample (-0x8000 - 0x7FFF) to this sample type.
	fn from_i16(v: i16) -> Self;
	/// Convert from an i24 sample (-0x800000 - 0x7FFFFF) to this sample type.
	fn from_i24(v: i24) -> Self;
	/// Convert from an i32 sample (-0x80000000 - 0x7FFFFFFF) to this sample type.
	fn from_i32(v: i32) -> Self;

	/// Convert from an f32 sample (-1.0 - 1.0) to this sample type.
	fn from_f32(v: f32) -> Self;
	/// Convert from an f64 sample (-1.0 - 1.0) to this sample type.
	fn from_f64(v: f64) -> Self;

	// The inverse of from_u8().
	fn to_u8(v: Self) -> u8;
	// The inverse of from_u16().
	fn to_u16(v: Self) -> u16;
	// The inverse of from_u24().
	fn to_u24(v: Self) -> u24;
	// The inverse of from_u32().
	fn to_u32(v: Self) -> u32;

	// The inverse of from_i8().
	fn to_i8(v: Self) -> i8;
	// The inverse of from_i16().
	fn to_i16(v: Self) -> i16;
	// The inverse of from_i24().
	fn to_i24(v: Self) -> i24;
	// The inverse of from_i32().
	fn to_i32(v: Self) -> i32;

	// The inverse of from_f32().
	fn to_f32(v: Self) -> f32;
	// The inverse of from_f64().
	fn to_f64(v: Self) -> f64;

	// Convert from raw little endian bytes.
	unsafe fn from_raw_le(ptr: *const u8) -> Self;
	// Convert from raw big endian bytes.
	unsafe fn from_raw_be(ptr: *const u8) -> Self;
	// Convert to raw little endian bytes.
	unsafe fn to_raw_le(v: Self, ptr: *mut u8);
	// Convert to raw big endian bytes.
	unsafe fn to_raw_be(v: Self, ptr: *mut u8);
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct u24(u32);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct i24(i32);

// These are provided to simplify the implementation of from_[ui]*() for f32 and f64.

impl u24 {
	#[inline]
	pub fn min_value() -> u32 {
		0x00000000
	}
	#[inline]
	pub fn max_value() -> u32 {
		0x00FFFFFF
	}
}

impl i24 {
	#[inline]
	pub fn min_value() -> i32 {
		-0x0080000
	}
	#[inline]
	pub fn max_value() -> i32 {
		0x007FFFFF
	}
}

macro_rules! impl_to_methods {
	($from_ty:ident) => {
		fn to_u8(v: Self) -> u8 { u8::$from_ty(v) }
		fn to_u16(v: Self) -> u16 { u16::$from_ty(v) }
		fn to_u24(v: Self) -> u24 { u24::$from_ty(v) }
		fn to_u32(v: Self) -> u32 { u32::$from_ty(v) }

		fn to_i8(v: Self) -> i8 { i8::$from_ty(v) }
		fn to_i16(v: Self) -> i16 { i16::$from_ty(v) }
		fn to_i24(v: Self) -> i24 { i24::$from_ty(v) }
		fn to_i32(v: Self) -> i32 { i32::$from_ty(v) }

		fn to_f32(v: Self) -> f32 { f32::$from_ty(v) }
		fn to_f64(v: Self) -> f64 { f64::$from_ty(v) }
	}
}

macro_rules! impl_raw_methods {
	() => {
		unsafe fn from_raw_le(ptr: *const u8) -> Self {
			Self::from_le(*(ptr as *const _))
		}
		unsafe fn from_raw_be(ptr: *const u8) -> Self {
			Self::from_be(*(ptr as *const _))
		}
		unsafe fn to_raw_le(v: Self, ptr: *mut u8) {
			*(ptr as *mut _) = Self::to_le(v);
		}
		unsafe fn to_raw_be(v: Self, ptr: *mut u8) {
			*(ptr as *mut _) = Self::to_be(v);
		}
	}
}

macro_rules! impl_raw_methods_24 {
	($ty_24:ident, $ty_32:ident) => {
		unsafe fn from_raw_le(ptr: *const u8) -> Self {
			// TODO: This seems like a suboptimal implementation.
			$ty_24(((u32::from_raw_le(ptr) << 8) as $ty_32) >> 8)
		}
		unsafe fn from_raw_be(ptr: *const u8) -> Self {
			// TODO: This seems like a suboptimal implementation.
			$ty_24(((u32::from_raw_le(ptr.offset(3)) << 8) as $ty_32) >> 8)
		}
		unsafe fn to_raw_le(v: Self, ptr: *mut u8) {
			*ptr = (v.0 & 0xFF) as u8;
			*ptr.offset(1) = ((v.0 >> 8) & 0xFF) as u8;
			*ptr.offset(2) = ((v.0 >> 16) & 0xFF) as u8;
		}
		unsafe fn to_raw_be(v: Self, ptr: *mut u8) {
			*ptr = ((v.0 >> 16) & 0xFF) as u8;
			*ptr.offset(1) = ((v.0 >> 8) & 0xFF) as u8;
			*ptr.offset(2) = (v.0 & 0xFF) as u8;
		}
	}
}

macro_rules! impl_float_methods {
	($ty:ident) => {
		// The +1.0 is to make the conversion lossless. Note that also means from_f32(1.0) will be invalid.
		fn from_f32(v: f32) -> Self {
			let x = 0.5 * ( (v + 1.0) * (($ty::max_value() as f32) + 1.0) +
			                (1.0 - v) * ($ty::min_value() as f32) );

			if x < $ty::min_value() as f32 {
				$ty::min_value()
			} else if x > $ty::max_value() as f32 {
				$ty::max_value()
			} else {
				x as _
			}
		}
		fn from_f64(v: f64) -> Self {
			let x = 0.5 * ( (v + 1.0) * (($ty::max_value() as f64) + 1.0) +
			                (1.0 - v) * ($ty::min_value() as f64) );

			if x < $ty::min_value() as f64 {
				$ty::min_value()
			} else if x > $ty::max_value() as f64 {
				$ty::max_value()
			} else {
				x as _
			}
		}
	}
}

macro_rules! impl_float_methods_24 {
	($ty:ident) => {
		fn from_f32(v: f32) -> Self {
			let x = 0.5 * ( (v + 1.0) * (($ty::max_value() as f32) + 1.0) +
							(1.0 - v) * ($ty::min_value() as f32) );
							
			if x < $ty::min_value() as f32 {
				$ty($ty::min_value())
			} else if x > $ty::max_value() as f32 {
				$ty($ty::max_value())
			} else {
				$ty(x as _)
			}
		}
		fn from_f64(v: f64) -> Self {
			let x = 0.5 * ( (v + 1.0) * (($ty::max_value() as f64) + 1.0) +
							(1.0 - v) * ($ty::min_value() as f64) );
							
			if x < $ty::min_value() as f64 {
				$ty($ty::min_value())
			} else if x > $ty::max_value() as f64 {
				$ty($ty::max_value())
			} else {
				$ty(x as _)
			}
		}
	}
}

macro_rules! impl_from_signed_methods {
	() => {
		fn from_i8(v: i8) -> Self { Self::from_u8((v as u8).wrapping_add(0x80)) }
		fn from_i16(v: i16) -> Self { Self::from_u16((v as u16).wrapping_add(0x8000)) }
		fn from_i24(v: i24) -> Self { Self::from_u24(u24((v.0 as u32).wrapping_add(0x800000) & 0x00FFFFFF)) }
		fn from_i32(v: i32) -> Self { Self::from_u32((v as u32).wrapping_add(0x80000000)) }
	}
}

macro_rules! impl_from_unsigned_methods {
	() => {
		fn from_u8(v: u8) -> Self { Self::from_i8(v.wrapping_add(0x80) as i8) }
		fn from_u16(v: u16) -> Self { Self::from_i16(v.wrapping_add(0x8000) as i16) }
		fn from_u24(v: u24) -> Self { Self::from_i24(i24((v.0.wrapping_add(0x800000) & 0x00FFFFFF) as i32)) }
		fn from_u32(v: u32) -> Self { Self::from_i32(v.wrapping_add(0x80000000) as i32) }
	}
}

impl Sample for u8 {
	fn from_u8(v: u8) -> Self { v }
	fn from_u16(v: u16) -> Self { (v >> 8) as _ }
	fn from_u24(v: u24) -> Self { (v.0 >> 16) as _ }
	fn from_u32(v: u32) -> Self { (v >> 24) as _ }

	impl_from_signed_methods!();
	impl_float_methods!(u8);
	impl_to_methods!(from_u8);
	impl_raw_methods!();
}

impl Sample for u16 {
	fn from_u8(v: u8) -> Self { (v as u16) << 8 }
	fn from_u16(v: u16) -> Self { v }
	fn from_u24(v: u24) -> Self { (v.0 >> 8) as _ }
	fn from_u32(v: u32) -> Self { (v >> 16) as _ }

	impl_from_signed_methods!();
	impl_float_methods!(u16);
	impl_to_methods!(from_u16);
	impl_raw_methods!();
}

impl Sample for u24 {
	fn from_u8(v: u8) -> Self { u24((v as u32) << 16) }
	fn from_u16(v: u16) -> Self { u24((v as u32) << 8) }
	fn from_u24(v: u24) -> Self { v }
	fn from_u32(v: u32) -> Self { u24(v >> 8) }

	impl_from_signed_methods!();
	impl_float_methods_24!(u24);
	impl_to_methods!(from_u24);
	impl_raw_methods_24!(u24, u32);
}

impl Sample for u32 {
	fn from_u8(v: u8) -> Self { (v as u32) << 24 }
	fn from_u16(v: u16) -> Self { (v as u32) << 16 }
	fn from_u24(v: u24) -> Self { v.0 << 8 }
	fn from_u32(v: u32) -> Self { v }

	impl_from_signed_methods!();
	impl_float_methods!(u32);
	impl_to_methods!(from_u32);
	impl_raw_methods!();
}

impl Sample for i8 {
	fn from_i8(v: i8) -> Self { v }
	fn from_i16(v: i16) -> Self { (v >> 8) as _ }
	fn from_i24(v: i24) -> Self { (v.0 >> 16) as _ }
	fn from_i32(v: i32) -> Self { (v >> 24) as _ }

	impl_from_unsigned_methods!();
	impl_float_methods!(i8);
	impl_to_methods!(from_i8);
	impl_raw_methods!();
}
impl Sample for i16 {
	fn from_i8(v: i8) -> Self { (v as i16) << 8 }
	fn from_i16(v: i16) -> Self { v }
	fn from_i24(v: i24) -> Self { (v.0 >> 8) as _ }
	fn from_i32(v: i32) -> Self { (v >> 16) as _ }

	impl_from_unsigned_methods!();
	impl_float_methods!(i16);
	impl_to_methods!(from_i16);
	impl_raw_methods!();
}

impl Sample for i24 {
	fn from_i8(v: i8) -> Self { i24((v as i32) << 16) }
	fn from_i16(v: i16) -> Self { i24((v as i32) << 8) }
	fn from_i24(v: i24) -> Self { v }
	fn from_i32(v: i32) -> Self { i24(v >> 8) }

	impl_from_unsigned_methods!();
	impl_float_methods_24!(i24);
	impl_to_methods!(from_i24);
	impl_raw_methods_24!(i24, i32);
}

impl Sample for i32 {
	fn from_i8(v: i8) -> Self { (v as i32) << 24 }
	fn from_i16(v: i16) -> Self { (v as i32) << 16 }
	fn from_i24(v: i24) -> Self { v.0 << 8 }
	fn from_i32(v: i32) -> Self { v }

	impl_from_unsigned_methods!();
	impl_float_methods!(i32);
	impl_to_methods!(from_i32);
	impl_raw_methods!();
}



macro_rules! impl_float_raw_methods {
	($uint_ty:ident) => {
		unsafe fn from_raw_le(ptr: *const u8) -> Self {
			std::mem::transmute($uint_ty::from_le(*(ptr as *const _)))
		}
		unsafe fn from_raw_be(ptr: *const u8) -> Self {
			std::mem::transmute($uint_ty::from_be(*(ptr as *const _)))
		}
		unsafe fn to_raw_le(v: Self, ptr: *mut u8) {
			*(ptr as *mut _) = $uint_ty::to_le(std::mem::transmute(v));
		}
		unsafe fn to_raw_be(v: Self, ptr: *mut u8) {
			*(ptr as *mut _) = $uint_ty::to_le(std::mem::transmute(v));
		}
	}
}

macro_rules! impl_float_from_methods {
	() => {
		// The way I have chosen to do conversions is to map -128:127 to -1.0:0.992
		// This way there is an exact correspondence between the mantissa bits and the integer bits.
		// In fact I could do this conversion with bit fiddling.
		// The mapping for unsigned is 0:255 to -1.0:0.992, so u8->f32->i8->u8 should be lossless.
		fn from_u8(v: u8) -> Self { (v as Self) / 128.0 - 1.0 }
		fn from_u16(v: u16) -> Self { (v as Self) / 32768.0 - 1.0 }
		fn from_u24(v: u24) -> Self { (v.0 as Self) / 8388608.0 - 1.0 }
		fn from_u32(v: u32) -> Self { (v as Self) / 2147483648.0 - 1.0 }

		fn from_i8(v: i8) -> Self { (v as Self) / 128.0 }
		fn from_i16(v: i16) -> Self { (v as Self) / 32768.0 }
		fn from_i24(v: i24) -> Self { (v.0 as Self) / 8388608.0 }
		fn from_i32(v: i32) -> Self { (v as Self) / 2147483648.0 }

		fn from_f32(v: f32) -> Self { v as Self }
		fn from_f64(v: f64) -> Self { v as Self }
	}
}

impl Sample for f32 {
	impl_float_from_methods!();
	impl_to_methods!(from_f32);
	impl_float_raw_methods!(u32);
}

impl Sample for f64 {
	impl_float_from_methods!();
	impl_to_methods!(from_f64);
	impl_float_raw_methods!(u64);
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sign_conversion_lossless() {
		// This doesn't test the max value but is there even a way to do that cleanly
		// without inclusive ranges?
		for v in u8::min_value()..u8::max_value() {
			assert_eq!(v, u8::from_i8(i8::from_u8(v)));
		}
		for v in u16::min_value()..u16::max_value() {
			assert_eq!(v, u16::from_i16(i16::from_u16(v)));
		}
		for v in u24::min_value()..u24::max_value() {
			assert_eq!(u24(v), u24::from_i24(i24::from_u24(u24(v))));
		}
	}

	#[test]
	fn increased_precision_lossless() {
		for v in u8::min_value()..u8::max_value() {
			assert_eq!(v, u8::from_i16(i16::from_u8(v)));
		}
		for v in u16::min_value()..u16::max_value() {
			assert_eq!(v, u16::from_i24(i24::from_u16(v)));
		}
		for v in u24::min_value()..u24::max_value() {
			assert_eq!(u24(v), u24::from_i32(i32::from_u24(u24(v))));
		}
	}

	#[test]
	fn float_lossless() {
		let mut v: f32 = -1.0;
		while v <= 1.0 {
			assert_eq!(v, f32::from_f64(f64::from_f32(v)));
			v += 0.01;
		}
	}

	#[test]
	fn float_int_conversions() {
		assert_eq!(f32::from_i8(-128), -1.0);
		assert_eq!(f32::from_i8(0), 0.0);
		assert_eq!(f32::from_i8(64), 0.5);

		assert_eq!(f32::from_u8(0), -1.0);
		assert_eq!(f32::from_u8(128), 0.0);
		assert_eq!(f32::from_u8(192), 0.5);
	}

	#[test]
	fn int_through_float_lossless() {
		for v in u8::min_value()..u8::max_value() {
			assert_eq!(v, u8::from_f32(f32::from_i16(i16::from_u8(v))));
		}
		for v in u16::min_value()..u16::max_value() {
			assert_eq!(v, u16::from_f64(f64::from_i24(i24::from_u16(v))));
		}
	}

	#[test]
	fn out_of_range_float() {
		assert_eq!(0, u8::from_f64(-1.0));
		assert_eq!(0, u8::from_f64(-10.0));
		assert_eq!(255, u8::from_f64(1.0));
		assert_eq!(255, u8::from_f64(10.0));

		assert_eq!(-128, i8::from_f64(-1.0));
		assert_eq!(-128, i8::from_f64(-10.0));
		assert_eq!(127, i8::from_f64(1.0));
		assert_eq!(127, i8::from_f64(10.0));
	}

	#[test]
	fn raw_lossless() {
		unsafe {
			let mut buffer = [0u8; 32];
			
			let ptr = &mut buffer[0] as *mut u8;

			for v in u8::min_value()..u8::max_value() {
				u8::to_raw_le(v, ptr);
				assert_eq!(v, u8::from_raw_le(ptr));
				assert_eq!(v.swap_bytes(), u8::from_raw_be(ptr));
			}
			for v in i8::min_value()..i8::max_value() {
				i8::to_raw_le(v, ptr);
				assert_eq!(v, i8::from_raw_le(ptr));
				assert_eq!(v.swap_bytes(), i8::from_raw_be(ptr));
			}
			for v in u16::min_value()..u16::max_value() {
				u16::to_raw_le(v, ptr);
				assert_eq!(v, u16::from_raw_le(ptr));
				assert_eq!(v.swap_bytes(), u16::from_raw_be(ptr));
			}
			for v in u24::min_value()..u24::max_value() {
				u24::to_raw_le(u24(v), ptr);
				assert_eq!(u24(v), u24::from_raw_le(ptr));
			}
			for v in i24::min_value()..i24::max_value() {
				i24::to_raw_le(i24(v), ptr);
				assert_eq!(i24(v), i24::from_raw_le(ptr));
			}
		}
	}
}
