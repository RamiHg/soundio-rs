extern crate libsoundio_sys as raw;

use std::ffi::CStr;
use std::fmt;
use std::error;
use std::result;

use std::os::raw::c_int;

/// Error is the error return type for many functions. These are
/// taken directly from libsoundio. It supports conversion to `String` using
/// the `From` trait.
///
/// # Examples
///
/// ```
/// use std::error::Error;
/// let e = soundio::Error::IncompatibleDevice;
/// println!("{}", e.description());
/// ```
#[derive(Debug, Copy, Clone)]
pub enum Error {
	/// Out of memory.
	NoMem,
	/// The backend does not appear to be active or running.
	InitAudioBackend,
	/// A system resource other than memory was not available.
	SystemResources,
	/// Attempted to open a device and failed.
	OpeningDevice,
	/// No device found.
	NoSuchDevice,
	/// The programmer did not comply with the API.
	Invalid,
	/// libsoundio was compiled without support for that backend.
	BackendUnavailable,
	/// An open stream had an error that can only be recovered from by
	/// destroying the stream and creating it again.
	Streaming,
	/// Attempted to use a device with parameters it cannot support.
	IncompatibleDevice,
	/// When JACK returns `JackNoSuchClient`
	NoSuchClient,
	/// Attempted to use parameters that the backend cannot support.
	IncompatibleBackend,
	/// Backend server shutdown or became inactive.
	BackendDisconnected,
	Interrupted,
	/// Buffer underrun occurred.
	Underflow,
	/// Unable to convert to or from UTF-8 to the native string format.
	EncodingString,
	/// Unknown error that libsoundio should never return.
	Unknown,
}

impl From<c_int> for Error {
    fn from(err: c_int) -> Error {
		match err {
			1 => Error::NoMem,
			2 => Error::InitAudioBackend,
			3 => Error::SystemResources,
			4 => Error::OpeningDevice,
			5 => Error::NoSuchDevice,
			6 => Error::Invalid,
			7 => Error::BackendUnavailable,
			8 => Error::Streaming,
			9 => Error::IncompatibleDevice,
			10 => Error::NoSuchClient,
			11 => Error::IncompatibleBackend,
			12 => Error::BackendDisconnected,
			13 => Error::Interrupted,
			14 => Error::Underflow,
			15 => Error::EncodingString,
			_ => Error::Unknown,
		}
    }
}

impl From<Error> for c_int {
	fn from(err: Error) -> c_int {
		match err {
			Error::NoMem => 1,
			Error::InitAudioBackend => 2,
			Error::SystemResources => 3,
			Error::OpeningDevice => 4,
			Error::NoSuchDevice => 5,
			Error::Invalid => 6,
			Error::BackendUnavailable => 7,
			Error::Streaming => 8,
			Error::IncompatibleDevice => 9,
			Error::NoSuchClient => 10,
			Error::IncompatibleBackend => 11,
			Error::BackendDisconnected => 12,
			Error::Interrupted => 13,
			Error::Underflow => 14,
			Error::EncodingString => 15,
			Error::Unknown => -1, // This should never happen really.
		}
	}
}

/// Local typedef for results that soundio-rs returns.
pub type Result<T> = result::Result<T, Error>;

// Implement displaying the error. We just use the description.
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::error::Error;
		f.write_str(self.description())
	}
}

// Implement the description for errors using soundio_strerror(), and the cause which we never know.
impl error::Error for Error {
	fn description(&self) -> &str {
		let c_str: &CStr = unsafe { CStr::from_ptr(raw::soundio_strerror((*self).into())) };

		// to_str() checks for valid UTF-8 since that what a &str is. For now at least there are
		// no invalid UTF-8 sequences in the C source.
		c_str.to_str().unwrap()
	}

	fn cause(&self) -> Option<&error::Error> {
		// We never have any more cause information unfortunately.
		None
	}
}

impl From<Error> for String {
    fn from(err: Error) -> String {
		use std::error::Error;
		err.description().to_string()
    }
}