extern crate libsoundio_sys as raw;

use super::error::*;
use super::format::*;
use super::util::*;
use super::sample::*;

use std::ptr;
use std::os::raw::{c_int, c_double};
use std::marker::PhantomData;
use std::slice;

/// This is called when an instream has been read. The `InStreamUserData` struct is obtained
/// from the stream.userdata, then the user-supplied callback is called with an `InStreamReader`
/// object.
pub extern fn instream_read_callback(stream: *mut raw::SoundIoInStream, frame_count_min: c_int, frame_count_max: c_int) {
	// Use stream.userdata to get a reference to the InStreamUserData object.
	let raw_userdata_pointer = unsafe { (*stream).userdata as *mut InStreamUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	let mut stream_reader = InStreamReader {
		instream: userdata.instream,
		frame_count_min: frame_count_min as _,
		frame_count_max: frame_count_max as _,
		read_started: false,
		channel_areas: Vec::new(),
		frame_count: 0,
		phantom: PhantomData,
	};

	(userdata.read_callback)(&mut stream_reader);
}

pub extern fn instream_overflow_callback(stream: *mut raw::SoundIoInStream) {
	// Use stream.userdata to get a reference to the InStreamUserData object.
	let raw_userdata_pointer = unsafe { (*stream).userdata as *mut InStreamUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	if let Some(ref mut cb) = userdata.overflow_callback {
		cb();
	} else {
		println!("Overflow!");
	}
}

pub extern fn instream_error_callback(stream: *mut raw::SoundIoInStream, err: c_int) {
	// Use stream.userdata to get a reference to the InStreamUserData object.
	let raw_userdata_pointer = unsafe { (*stream).userdata as *mut InStreamUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	if let Some(ref mut cb) = userdata.error_callback {
		cb(err.into());
	} else {
		println!("Error: {}", Error::from(err));
	}
}

/// InStream represents an input stream for recording.
///
/// It is obtained from `Device` using `Device::open_instream()` and
/// can be started and paused.
pub struct InStream<'a> {
	pub userdata: Box<InStreamUserData<'a>>,
	
	// This is just here to say that InStream cannot outlive the Device it was created from.
	pub phantom: PhantomData<&'a ()>,
}

/// The callbacks required for an instream are stored in this object. We also store a pointer
/// to the raw instream so that it can be passed to `InStreamReader` in the write callback.
pub struct InStreamUserData<'a> {
	pub instream: *mut raw::SoundIoInStream,

	pub read_callback: Box<FnMut(&mut InStreamReader) + 'a>,
	pub overflow_callback: Option<Box<FnMut() + 'a>>,
	pub error_callback: Option<Box<FnMut(Error) + 'a>>,
}

impl<'a> Drop for InStreamUserData<'a> {
	fn drop(&mut self) {
		unsafe {
			raw::soundio_instream_destroy(self.instream);
		}
	}
}

impl<'a> InStream<'a> {
	/// Starts the stream, returning `Ok(())` if it started successfully. Once
	/// started the read callback will be periodically called according to the
	/// requested latency.
	///
	/// `start()` should only ever be called once on an `InStream`.
	/// Do not use `start()` to resume a stream after pausing it. Instead call `pause(false)`.
	///
	/// # Errors
	///
	/// * `Error::BackendDisconnected`
	/// * `Error::Streaming`
	/// * `Error::OpeningDevice`
	/// * `Error::SystemResources`
	///
	pub fn start(&mut self) -> Result<()> {
		match unsafe { raw::soundio_instream_start(self.userdata.instream) } {
			0 => Ok(()),
			x => Err(x.into()),
		}
	}

	// TODO: Can pause() be called from the read callback?

	/// If the underlying backend and device support pausing, this pauses the
	/// stream. The `write_callback()` may be called a few more times if
	/// the buffer is not full.
	///
	/// Pausing might put the hardware into a low power state which is ideal if your
	/// software is silent for some time.
	///
	/// This should not be called before `start()`. Pausing when already paused or
	/// unpausing when already unpaused has no effect and returns `Ok(())`.
	///
	/// # Errors
	///
	/// * `Error::BackendDisconnected`
	/// * `Error::Streaming`
	/// * `Error::IncompatibleDevice` - device does not support pausing/unpausing
	///
	pub fn pause(&mut self, pause: bool) -> Result<()> {
		match unsafe { raw::soundio_instream_pause(self.userdata.instream, pause as i8) } {
			0 => Ok(()),
			e => Err(e.into()),
		}
	}

	/// Returns the stream format.
	pub fn format(&self) -> Format {
		unsafe {
			(*self.userdata.instream).format.into()
		}
	}

	/// Sample rate is the number of frames per second.
	pub fn sample_rate(&self) -> i32 {
		unsafe {
			(*self.userdata.instream).sample_rate as _
		}		
	}

    /// Ignoring hardware latency, this is the number of seconds it takes for a
    /// captured sample to become available for reading.
    /// After you call `Device::open_instream()`, this value is replaced with the
    /// actual software latency, as near to this value as possible.
	///
    /// A higher value means less CPU usage. Defaults to a large value,
    /// potentially upwards of 2 seconds.
	///
    /// If the device has unknown software latency min and max values, you may
    /// still set this (in `Device::open_instream()`), but you might not
	/// get the value you requested.
	///
    /// For PulseAudio, if you set this value to non-default, it sets
    /// `PA_STREAM_ADJUST_LATENCY` and is the value used for `fragsize`.
    /// For JACK, this value is always equal to
    /// `Device::software_latency().current`.
	pub fn software_latency(&self) -> f64 {
		unsafe {
			(*self.userdata.instream).software_latency as _
		}
	}

	/// The name of the stream, which defaults to "SoundIoInStream".
	///
    /// PulseAudio uses this for the stream name.
    /// JACK uses this for the client name of the client that connects when you
    /// open the stream.
    /// WASAPI uses this for the session display name.
    /// Must not contain a colon (":").
	///
	/// TODO: Currently there is no way to set this.
	pub fn name(&self) -> String {
		unsafe {
			utf8_to_string((*self.userdata.instream).name)
		}
	}

	/// The number of bytes per frame, equal to the number of bytes
	/// per sample, multiplied by the number of channels.
	pub fn bytes_per_frame(&self) -> i32 {
		unsafe {
			(*self.userdata.instream).bytes_per_frame as _
		}
	}

	/// The number of bytes in a sample, e.g. 3 for `i24`.
	pub fn bytes_per_sample(&self) -> i32 {
		unsafe {
			(*self.userdata.instream).bytes_per_sample as _
		}
	}
}

/// `InStreamReader` is passed to the read callback and can be used to read from the stream.
///
/// You start by calling `begin_read()` and then you can read the samples. When the `InStreamReader`
/// is dropped the samples are dropped. An error at that point is written to the console and ignored.
///
pub struct InStreamReader<'a> {
	instream: *mut raw::SoundIoInStream,
	frame_count_min: usize,
	frame_count_max: usize,

	read_started: bool,

	// The memory area to write to - one for each channel. Populated after begin_read()
	channel_areas: Vec<raw::SoundIoChannelArea>,
	// The actual frame count. Populated after begin_read()
	frame_count: usize,

	// This cannot outlive the scope that it is spawned from (in the write callback).
	phantom: PhantomData<&'a ()>,
}

impl<'a> InStreamReader<'a> {
	/// Start a read. You can only call this once per callback otherwise it panics.
	///
	/// frame_count is the number of frames you want to read. It must be between
	/// frame_count_min and frame_count_max inclusive, or `begin_read()` will panic.
	///
	/// It returns the number of frames you can actually read. The returned value
	/// will always be less than or equal to the provided value.
	///
	/// # Errors
	///
	/// * `Error::Invalid`
	///   * `frame_count` < `frame_count_min` or `frame_count` > `frame_count_max`
	/// * `Error::Streaming`
	/// * `Error::IncompatibleDevice` - in rare cases it might just now
	///   be discovered that the device uses non-byte-aligned access, in which
	///   case this error code is returned.
	///
	pub fn begin_read(&mut self, frame_count: usize) -> Result<usize> {
		assert!(frame_count >= self.frame_count_min && frame_count <= self.frame_count_max, "frame_count out of range");

		let mut areas: *mut raw::SoundIoChannelArea = ptr::null_mut();
		let mut actual_frame_count: c_int = frame_count as _;

		match unsafe { raw::soundio_instream_begin_read(self.instream, &mut areas as *mut _, &mut actual_frame_count as *mut _) } {
			0 => {
				self.read_started = true;
				self.frame_count = actual_frame_count as _;
				// Return now if there's no frames to actually read.
				if actual_frame_count <= 0 {
					return Ok(0);
				}
				let cc = self.channel_count();
				self.channel_areas = vec![raw::SoundIoChannelArea { ptr: ptr::null_mut(), step: 0 }; cc];
				unsafe { self.channel_areas.copy_from_slice(slice::from_raw_parts::<raw::SoundIoChannelArea>(areas, cc)); }
				Ok(actual_frame_count as _)
			},
			e => Err(e.into()),
		}
	}

	/// Commits the write that you began with `begin_read()`.
	///
	/// Errors are currently are just printed to the console and ignored.
	///
	/// # Errors
	///
	/// * `Error::Streaming`
	/// * `Error::Underflow` - an underflow caused this call to fail. You might
	///   also get an `underflow_callback()`, and you might not get
	///   this error code when an underflow occurs. Unlike `Error::Streaming`,
	///   the outstream is still in a valid state and streaming can continue.
	pub fn end_read(&mut self) {
		if self.read_started {
			unsafe {
				match raw::soundio_instream_end_read(self.instream) {
					0 => {self.read_started = false;},
					x => println!("Error ending instream: {}", Error::from(x)),
				}
			}
		}
	}
	
	/// Get the minimum frame count that you can call `begin_read()` with.
	/// Retreive this value before calling `begin_read()` to ensure you read the correct number
	/// of frames.
	pub fn frame_count_min(&self) -> usize {
		self.frame_count_min
	}

	/// Get the maximum frame count that you can call `begin_read()` with.
	/// Retreive this value before calling `begin_read()` to ensure you read the correct number
	/// of frames.
	pub fn frame_count_max(&self) -> usize {
		self.frame_count_max
	}

	/// Get the actual frame count that you did call `begin_read()` with. Panics if you haven't called
	/// `begin_read()` yet.
	pub fn frame_count(&self) -> usize {
		assert!(self.read_started);
		self.frame_count
	}

	/// Get latency in seconds due to software only, not including hardware.
	pub fn software_latency(&self) -> f64 {
		unsafe {
			(*self.instream).software_latency as _
		}
	}

	/// Return the number of channels in this stream. Guaranteed to be at least 1.
	pub fn channel_count(&self) -> usize {
		unsafe {
			(*self.instream).layout.channel_count as _
		}
	}

	/// Get the sample rate in Hertz.
	pub fn sample_rate(&self) -> i32 {
		unsafe {
			(*self.instream).sample_rate as _
		}
	}

	/// Obtain the number of seconds that the next frame of sound being
	/// captured will take to arrive in the buffer, plus the amount of time that is
	/// represented in the buffer. This includes both software and hardware latency.
	///
	/// # Errors
	///
	/// * `Error::Streaming`
	///
	pub fn get_latency(&mut self) -> Result<f64> {
		let mut x: c_double = 0.0;
		match unsafe { raw::soundio_instream_get_latency(self.instream, &mut x as *mut c_double) } {
			0 => Ok(x),
			e => Err(e.into()),
		}
	}

	/// Get the value of a sample. This panics if the `channel` or `frame` are
	/// out of range or if you haven't called `begin_read()` yet.
	///
	/// If you request a different type from the actual one it will be converted.
	///
	/// # Examples
	///
	/// ```
	/// fn read_callback(stream: &mut soundio::InStreamReader) {
	///     let frame_count_max = stream.frame_count_max();
	///     stream.begin_read(frame_count_max).unwrap();
	///     for c in 0..stream.channel_count() {
	///         for f in 0..stream.frame_count() {
	///             do_something_with(stream.sample::<i16>(c, f));
	///         }
	///     }
	/// }
	/// # fn do_something_with(_: i16) { }
	/// ```
	pub fn sample<T: Sample>(&self, channel: usize, frame: usize) -> T {
		assert!(self.read_started);

		assert!(channel < self.channel_count(), "Channel out of range");
		assert!(frame < self.frame_count(), "Frame out of range");

		unsafe {
			let ptr = self.channel_areas[channel].ptr.offset((frame * self.channel_areas[channel].step as usize) as isize) as *mut u8;

			match (*self.instream).format {
				raw::SoundIoFormat::SoundIoFormatS8 => T::from_i8(i8::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatU8 => T::from_u8(u8::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatS16LE => T::from_i16(i16::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatS16BE => T::from_i16(i16::from_raw_be(ptr)),
				raw::SoundIoFormat::SoundIoFormatU16LE => T::from_u16(u16::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatU16BE => T::from_u16(u16::from_raw_be(ptr)),
				raw::SoundIoFormat::SoundIoFormatS24LE => T::from_i24(i24::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatS24BE => T::from_i24(i24::from_raw_be(ptr)),
				raw::SoundIoFormat::SoundIoFormatU24LE => T::from_u24(u24::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatU24BE => T::from_u24(u24::from_raw_be(ptr)),
				raw::SoundIoFormat::SoundIoFormatS32LE => T::from_i32(i32::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatS32BE => T::from_i32(i32::from_raw_be(ptr)),
				raw::SoundIoFormat::SoundIoFormatU32LE => T::from_u32(u32::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatU32BE => T::from_u32(u32::from_raw_be(ptr)),
				raw::SoundIoFormat::SoundIoFormatFloat32LE => T::from_f32(f32::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatFloat32BE => T::from_f32(f32::from_raw_be(ptr)),
				raw::SoundIoFormat::SoundIoFormatFloat64LE => T::from_f64(f64::from_raw_le(ptr)),
				raw::SoundIoFormat::SoundIoFormatFloat64BE => T::from_f64(f64::from_raw_be(ptr)),
				_ => panic!("Unknown format"),			
			}
		}
	}

	// TODO: To acheive speed *and* safety I can use iterators. That will be in a future API.
}

impl<'a> Drop for InStreamReader<'a> {
	/// This will drop all of the frames from when you called `begin_read()`.
	///
	/// Errors are currently are just printed to the console and ignored.
	///
	/// # Errors
	///
	/// * `Error::Streaming`
	fn drop(&mut self) {
		if self.read_started {
			unsafe {
				match raw::soundio_instream_end_read(self.instream) {
					0 => {},
					x => println!("Error reading instream: {}", Error::from(x)),
				}
			}
		}
	}
}
