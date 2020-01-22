extern crate libsoundio_sys as raw;

use std::ffi::CStr;
use std::fmt;

/// Format defines the format of the samples. In 90% of cases you'll want `S16LE`, or maybe `Float64LE`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Format {
	/// Invalid format
	Invalid,
	/// Signed 8 bit
	S8,
	/// Unsigned 8 bit
	U8,
	/// Signed 16 bit Little Endian
	S16LE,
	/// Signed 16 bit Big Endian
	S16BE,
	/// Unsigned 16 bit Little Endian
	U16LE,
	/// Unsigned 16 bit Big Endian
	U16BE,
	/// Signed 24 bit Little Endian using low three bytes in 32-bit word
	S24LE,
	/// Signed 24 bit Big Endian using low three bytes in 32-bit word
	S24BE,
	/// Unsigned 24 bit Little Endian using low three bytes in 32-bit word
	U24LE,
	/// Unsigned 24 bit Big Endian using low three bytes in 32-bit word
	U24BE,
	/// Signed 32 bit Little Endian
	S32LE,
	/// Signed 32 bit Big Endian
	S32BE,
	/// Unsigned 32 bit Little Endian
	U32LE,
	/// Unsigned 32 bit Big Endian
	U32BE,
	/// Float 32 bit Little Endian, Range -1.0 to 1.0
	Float32LE,
	/// Float 32 bit Big Endian, Range -1.0 to 1.0
	Float32BE,
	/// Float 64 bit Little Endian, Range -1.0 to 1.0
	Float64LE,
	/// Float 64 bit Big Endian, Range -1.0 to 1.0
	Float64BE,
}

/// This is a small helper type to find the endianness of a sample format.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Endian {
	Big,
	Little,
}

/// Return the endianness of a sample format. `Format::Invalid`,
/// `Format::S8` and `Format::U8` return `Endian::Little`.
///
/// # Examples
///
/// ```
/// use soundio::*;
/// assert_eq!(endianness(Format::S24LE), Endian::Little);
/// assert_eq!(endianness(Format::U8), Endian::Little);
/// assert_eq!(endianness(Format::Float64BE), Endian::Big);
/// ```
pub fn endianness(f: Format) -> Endian {
	match f {
		Format::Invalid | Format::U8 | Format::S8 | Format::U16LE | Format::S16LE | Format::U24LE | Format::S24LE | Format::U32LE | Format::S32LE | Format::Float32LE | Format::Float64LE => Endian::Little,
		                                            Format::U16BE | Format::S16BE | Format::U24BE | Format::S24BE | Format::U32BE | Format::S32BE | Format::Float32BE | Format::Float64BE => Endian::Big,
	}
}

/// This module provides some aliases for native endian formats, and foreign endian formats.
#[cfg(target_endian = "big")]
#[allow(non_upper_case_globals)]
pub mod native {
	use super::Format;

	/// Signed 16 bit Foreign Endian
	pub const S16FE: Format = Format::S16LE;
	/// Signed 16 bit Native Endian
	pub const S16NE: Format = Format::S16BE;
	/// Unsigned 16 bit Foreign Endian
	pub const U16FE: Format = Format::U16LE;
	/// Unsigned 16 bit Native Endian
	pub const U16NE: Format = Format::U16BE;
	/// Signed 24 bit Foreign Endian using low three bytes in 32-bit word
	pub const S24FE: Format = Format::S24LE;
	/// Signed 24 bit Native Endian using low three bytes in 32-bit word
	pub const S24NE: Format = Format::S24BE;
	/// Unsigned 24 bit Foreign Endian using low three bytes in 32-bit word
	pub const U24FE: Format = Format::U24LE;
	/// Unsigned 24 bit Native Endian using low three bytes in 32-bit word
	pub const U24NE: Format = Format::U24BE;
	/// Signed 32 bit Foreign Endian
	pub const S32FE: Format = Format::S32LE;
	/// Signed 32 bit Native Endian
	pub const S32NE: Format = Format::S32BE;
	/// Unsigned 32 bit Foreign Endian
	pub const U32FE: Format = Format::U32LE;
	/// Unsigned 32 bit Native Endian
	pub const U32NE: Format = Format::U32BE;
	/// Float 32 bit Foreign Endian, Range -1.0 to 1.0
	pub const Float32FE: Format = Format::Float32LE;
	/// Float 32 bit Native Endian, Range -1.0 to 1.0
	pub const Float32NE: Format = Format::Float32BE;
	/// Float 64 bit Foreign Endian, Range -1.0 to 1.0
	pub const Float64FE: Format = Format::Float64LE;
	/// Float 64 bit Native Endian, Range -1.0 to 1.0
	pub const Float64NE: Format = Format::Float64BE;
}

/// This module provides some aliases for native endian formats, and foreign endian formats.
#[cfg(target_endian = "little")]
#[allow(non_upper_case_globals)]
pub mod native {
	use super::Format;

	/// Signed 16 bit Native Endian
	pub const S16NE: Format = Format::S16LE;
	/// Signed 16 bit Foreign Endian
	pub const S16FE: Format = Format::S16BE;
	/// Unsigned 16 bit Native Endian
	pub const U16NE: Format = Format::U16LE;
	/// Unsigned 16 bit Foreign Endian
	pub const U16FE: Format = Format::U16BE;
	/// Signed 24 bit Native Endian using low three bytes in 32-bit word
	pub const S24NE: Format = Format::S24LE;
	/// Signed 24 bit Foreign Endian using low three bytes in 32-bit word
	pub const S24FE: Format = Format::S24BE;
	/// Unsigned 24 bit Native Endian using low three bytes in 32-bit word
	pub const U24NE: Format = Format::U24LE;
	/// Unsigned 24 bit Foreign Endian using low three bytes in 32-bit word
	pub const U24FE: Format = Format::U24BE;
	/// Signed 32 bit Native Endian
	pub const S32NE: Format = Format::S32LE;
	/// Signed 32 bit Foreign Endian
	pub const S32FE: Format = Format::S32BE;
	/// Unsigned 32 bit Native Endian
	pub const U32NE: Format = Format::U32LE;
	/// Unsigned 32 bit Foreign Endian
	pub const U32FE: Format = Format::U32BE;
	/// Float 32 bit Native Endian, Range -1.0 to 1.0
	pub const Float32NE: Format = Format::Float32LE;
	/// Float 32 bit Foreign Endian, Range -1.0 to 1.0
	pub const Float32FE: Format = Format::Float32BE;
	/// Float 64 bit Native Endian, Range -1.0 to 1.0
	pub const Float64NE: Format = Format::Float64LE;
	/// Float 64 bit Foreign Endian, Range -1.0 to 1.0
	pub const Float64FE: Format = Format::Float64BE;
}

impl From<raw::SoundIoFormat> for Format {
	fn from(format: raw::SoundIoFormat) -> Format {
		match format {
			raw::SoundIoFormat::SoundIoFormatS8 => Format::S8,
			raw::SoundIoFormat::SoundIoFormatU8 => Format::U8,
			raw::SoundIoFormat::SoundIoFormatS16LE => Format::S16LE,
			raw::SoundIoFormat::SoundIoFormatS16BE => Format::S16BE,
			raw::SoundIoFormat::SoundIoFormatU16LE => Format::U16LE,
			raw::SoundIoFormat::SoundIoFormatU16BE => Format::U16BE,
			raw::SoundIoFormat::SoundIoFormatS24LE => Format::S24LE,
			raw::SoundIoFormat::SoundIoFormatS24BE => Format::S24BE,
			raw::SoundIoFormat::SoundIoFormatU24LE => Format::U24LE,
			raw::SoundIoFormat::SoundIoFormatU24BE => Format::U24BE,
			raw::SoundIoFormat::SoundIoFormatS32LE => Format::S32LE,
			raw::SoundIoFormat::SoundIoFormatS32BE => Format::S32BE,
			raw::SoundIoFormat::SoundIoFormatU32LE => Format::U32LE,
			raw::SoundIoFormat::SoundIoFormatU32BE => Format::U32BE,
			raw::SoundIoFormat::SoundIoFormatFloat32LE => Format::Float32LE,
			raw::SoundIoFormat::SoundIoFormatFloat32BE => Format::Float32BE,
			raw::SoundIoFormat::SoundIoFormatFloat64LE => Format::Float64LE,
			raw::SoundIoFormat::SoundIoFormatFloat64BE => Format::Float64BE,
			_ => Format::Invalid,
		}
	}
}

impl From<Format> for raw::SoundIoFormat {
	fn from(format: Format) -> raw::SoundIoFormat {
		match format {
			Format::S8 => raw::SoundIoFormat::SoundIoFormatS8,
			Format::U8 => raw::SoundIoFormat::SoundIoFormatU8,
			Format::S16LE => raw::SoundIoFormat::SoundIoFormatS16LE,
			Format::S16BE => raw::SoundIoFormat::SoundIoFormatS16BE,
			Format::U16LE => raw::SoundIoFormat::SoundIoFormatU16LE,
			Format::U16BE => raw::SoundIoFormat::SoundIoFormatU16BE,
			Format::S24LE => raw::SoundIoFormat::SoundIoFormatS24LE,
			Format::S24BE => raw::SoundIoFormat::SoundIoFormatS24BE,
			Format::U24LE => raw::SoundIoFormat::SoundIoFormatU24LE,
			Format::U24BE => raw::SoundIoFormat::SoundIoFormatU24BE,
			Format::S32LE => raw::SoundIoFormat::SoundIoFormatS32LE,
			Format::S32BE => raw::SoundIoFormat::SoundIoFormatS32BE,
			Format::U32LE => raw::SoundIoFormat::SoundIoFormatU32LE,
			Format::U32BE => raw::SoundIoFormat::SoundIoFormatU32BE,
			Format::Float32LE => raw::SoundIoFormat::SoundIoFormatFloat32LE,
			Format::Float32BE => raw::SoundIoFormat::SoundIoFormatFloat32BE,
			Format::Float64LE => raw::SoundIoFormat::SoundIoFormatFloat64LE,
			Format::Float64BE => raw::SoundIoFormat::SoundIoFormatFloat64BE,
			_ => raw::SoundIoFormat::SoundIoFormatInvalid,
		}
	}
}

impl Format {
	/// Returns the number of byte used per sample. Note that this
	/// is the size of the storage used for the sample, not the number of
	/// bits used. For example S24LE returns 4.
	///
	/// The returned values are specifically:
	///
	/// * S8: 1 
	/// * U8: 1 
	/// * S16LE: 2 
	/// * S16BE: 2 
	/// * U16LE: 2 
	/// * U16BE: 2 
	/// * S24LE: 4 
	/// * S24BE: 4 
	/// * U24LE: 4 
	/// * U24BE: 4 
	/// * S32LE: 4 
	/// * S32BE: 4 
	/// * U32LE: 4 
	/// * U32BE: 4 
	/// * Float32LE: 4 
	/// * Float32BE: 4 
	/// * Float64LE: 8 
	/// * Float64BE: 8 
	/// * Invalid: -1
	///
	/// # Examples
	///
	/// ```
	/// let a = soundio::Format::S8;
	/// assert_eq!(a.bytes_per_sample(), 1);
	/// let b = soundio::Format::Float64LE;
	/// assert_eq!(b.bytes_per_sample(), 8);
	/// ```
	pub fn bytes_per_sample(&self) -> usize {
		unsafe { raw::soundio_get_bytes_per_sample((*self).into()) as usize }
	}

	/// Returns the number of bytes per frame.
	/// A frame is one sample for all channels so this is simply the number
	/// of bytes for a sample `bytes_per_sample()` multiplied by the number of channels.
	///
	/// # Examples
	///
	/// ```
	/// let a = soundio::Format::S8;
	/// assert_eq!(a.bytes_per_frame(2), 2);
	/// let b = soundio::Format::Float64LE;
	/// assert_eq!(b.bytes_per_frame(4), 32);
	/// ```
	pub fn bytes_per_frame(&self, channel_count: usize) -> usize {
		self.bytes_per_sample() * channel_count
	}

	/// Returns the number of bytes per second, which is the number of bytes
	/// per frame multiplied by the number of frames per second (the sample rate).
	///
	/// # Examples
	///
	/// ```
	/// let a = soundio::Format::S8;
	/// assert_eq!(a.bytes_per_second(2, 8000), 16000);
	/// let b = soundio::Format::Float64LE;
	/// assert_eq!(b.bytes_per_second(4, 4000), 128000);
	/// ```
	pub fn bytes_per_second(&self, channel_count: usize, sample_rate: usize) -> usize {
		self.bytes_per_sample() * channel_count * sample_rate
	}
}

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let c_str: &CStr = unsafe { CStr::from_ptr(raw::soundio_format_string((*self).into())) };

		// These are all ASCII. See https://github.com/andrewrk/libsoundio/blob/323fb1aa277674e2eb126234e3e6edf10ee45461/src/soundio.c#L76
		f.write_str(c_str.to_str().unwrap())
	}
}
