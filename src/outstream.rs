extern crate libsoundio_sys as raw;

use super::error::*;
use super::format::*;
use super::util::*;
use super::sample::*;

use std::ptr;
use std::os::raw::{c_int, c_double};
use std::marker::PhantomData;
use std::slice;

/// This is called when an outstream needs to be written to. The `OutStreamUserData` struct is obtained
/// from the stream.userdata, then the user-supplied callback is called with an `OutStreamWriter`
/// object.
pub extern fn outstream_write_callback(stream: *mut raw::SoundIoOutStream, frame_count_min: c_int, frame_count_max: c_int) {
	// Use stream.userdata to get a reference to the OutStreamUserData object.
	let raw_userdata_pointer = unsafe { (*stream).userdata as *mut OutStreamUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	let mut stream_writer = OutStreamWriter {
		outstream: userdata.outstream,
		frame_count_min: frame_count_min as _,
		frame_count_max: frame_count_max as _,
		write_started: false,
		channel_areas: Vec::new(),
		frame_count: 0,
		phantom: PhantomData,
	};

	(userdata.write_callback)(&mut stream_writer);
}

pub extern fn outstream_underflow_callback(stream: *mut raw::SoundIoOutStream) {
	// Use stream.userdata to get a reference to the OutStreamUserData object.
	let raw_userdata_pointer = unsafe { (*stream).userdata as *mut OutStreamUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	if let Some(ref mut cb) = userdata.underflow_callback {
		cb();
	} else {
		println!("Underflow!");
	}
}

pub extern fn outstream_error_callback(stream: *mut raw::SoundIoOutStream, err: c_int) {
	// Use stream.userdata to get a reference to the OutStreamUserData object.
	let raw_userdata_pointer = unsafe { (*stream).userdata as *mut OutStreamUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	if let Some(ref mut cb) = userdata.error_callback {
		cb(err.into());
	} else {
		println!("Error: {}", Error::from(err));
	}
}

/// OutStream represents an output stream for playback.
///
/// It is obtained from `Device` using `Device::open_outstream()` and
/// can be started and paused.
pub struct OutStream<'a> {
	pub userdata: Box<OutStreamUserData<'a>>,
	
	// This is just here to say that OutStream cannot outlive the Device it was created from.
	pub phantom: PhantomData<&'a ()>,
}

// The callbacks required for an outstream are stored in this object. We also store a pointer
// to the raw outstream so that it can be passed to `OutStreamWriter` in the write callback.
pub struct OutStreamUserData<'a> {
	pub outstream: *mut raw::SoundIoOutStream,

	pub write_callback: Box<FnMut(&mut OutStreamWriter) + 'a>,
	pub underflow_callback: Option<Box<FnMut() + 'a>>,
	pub error_callback: Option<Box<FnMut(Error) + 'a>>,
}

impl<'a> Drop for OutStreamUserData<'a> {
	fn drop(&mut self) {
		unsafe {
			raw::soundio_outstream_destroy(self.outstream);
		}
	}
}

impl<'a> OutStream<'a> {
	/// Starts the stream, returning `Ok(())` if it started successfully. Once
	/// started the write callback will be periodically called according to the
	/// requested latency.
	///
	/// `start()` should only ever be called once on an `OutStream`.
	/// Do not use `start()` to resume a stream after pausing it. Instead call `pause(false)`.
	/// 
	/// This function might directly call the write callback.
	///
	/// # Errors
	///
	/// * `Error::Streaming`
	/// * `Error::NoMem`
	/// * `Error::SystemResources`
	/// * `Error::BackendDisconnected`
	///
	pub fn start(&mut self) -> Result<()> {
		match unsafe { raw::soundio_outstream_start(self.userdata.outstream) } {
			0 => Ok(()),
			x => Err(x.into()),
		}
	}

	/// Clears the output stream buffer.
	///
	/// This function can be called regardless of whether the outstream is paused
	/// or not. Some backends do not support clearing the buffer. On these backends this
	/// function will return `Error::IncompatibleBackend`.
	///
	/// Some devices do not support clearing the buffer. On these devices this
	/// function might return `Error::IncompatibleDevice`.
	///
	/// # Errors
	///
	/// * `Error::Streaming`
	/// * `Error::IncompatibleBackend`
	/// * `Error::IncompatibleDevice`
	///
	pub fn clear_buffer(&mut self) -> Result<()> {
		match unsafe { raw::soundio_outstream_clear_buffer(self.userdata.outstream) } {
			0 => Ok(()),
			e => Err(e.into()),
		}
	}

	// TODO: pause() can be called from the write callback, so add it to 
	// OutStreamWriter.

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
	/// * `Error::IncompatibleDevice` - device does not support
	///    pausing/unpausing. This error code might not be returned even if the
	///    device does not support pausing/unpausing.
	/// * `Error::IncompatibleBackend` - backend does not support
	///    pausing/unpausing.
	/// * `Error::Invalid` - outstream not opened and started
	///
	pub fn pause(&mut self, pause: bool) -> Result<()> {
		match unsafe { raw::soundio_outstream_pause(self.userdata.outstream, pause as i8) } {
			0 => Ok(()),
			e => Err(e.into()),
		}
	}

	/// Returns the stream format.
	pub fn format(&self) -> Format {
		unsafe {
			(*self.userdata.outstream).format.into()
		}
	}

	/// Sample rate is the number of frames per second.
	pub fn sample_rate(&self) -> i32 {
		unsafe {
			(*self.userdata.outstream).sample_rate as _
		}		
	}

	/// Ignoring hardware latency, this is the number of seconds it takes for
	/// the last sample in a full buffer to be played.
	/// After you call `Device::open_instream()`, this value is replaced with the
	/// actual software latency, as near to this value as possible.
	///
	/// On systems that support clearing the buffer, this defaults to a large
	/// latency, potentially upwards of 2 seconds, with the understanding that
	/// you will call `clear_buffer()` when you want to reduce
	/// the latency to 0. On systems that do not support clearing the buffer,
	/// this defaults to a reasonable lower latency value.
	///
	/// On backends with high latencies (such as 2 seconds), `frame_count_min`
	/// will be 0, meaning you don't have to fill the entire buffer. In this
	/// case, the large buffer is there if you want it; you only have to fill
	/// as much as you want. On backends like JACK, `frame_count_min` will be
	/// equal to `frame_count_max` and if you don't fill that many frames, you
	/// will get glitches.
	///
	/// If the device has unknown software latency min and max values, you may
	/// still set this (in `Device::open_outstream()`), but you might not get
	/// the value you requested.
	/// For PulseAudio, if you set this value to non-default, it sets
	/// `PA_STREAM_ADJUST_LATENCY` and is the value used for `maxlength` and
	/// `tlength`.
	///
	/// For JACK, this value is always equal to
	/// `Device::software_latency().current`.
	pub fn software_latency(&self) -> f64 {
		unsafe {
			(*self.userdata.outstream).software_latency as _
		}
	}

	/// The name of the stream, which defaults to "SoundIoOutStream".
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
			utf8_to_string((*self.userdata.outstream).name)
		}
	}

	/// The number of bytes per frame, equal to the number of bytes
	/// per sample, multiplied by the number of channels.
	pub fn bytes_per_frame(&self) -> i32 {
		unsafe {
			(*self.userdata.outstream).bytes_per_frame as _
		}
	}

	/// The number of bytes in a sample, e.g. 3 for `i24`.
	pub fn bytes_per_sample(&self) -> i32 {
		unsafe {
			(*self.userdata.outstream).bytes_per_sample as _
		}
	}
}

/// `OutStreamWriter` is passed to the write callback and can be used to write to the stream.
///
/// You start by calling `begin_write()` then you can write the samples. When the `OutStreamWriter``
/// is dropped the write is committed. An error at that point is written to the console and ignored.
///
pub struct OutStreamWriter<'a> {
	outstream: *mut raw::SoundIoOutStream,
	frame_count_min: usize,
	frame_count_max: usize,

	write_started: bool,

	// The memory area to write to - one for each channel. Populated after begin_write()
	channel_areas: Vec<raw::SoundIoChannelArea>,
	// The actual frame count. Populated after begin_write()
	frame_count: usize,

	// This cannot outlive the scope that it is spawned from (in the write callback).
	phantom: PhantomData<&'a ()>,
}

impl<'a> OutStreamWriter<'a> {
	/// Start a write. You can only call this once per callback otherwise it panics.
	///
	/// frame_count is the number of frames you want to write. It must be between
	/// frame_count_min and frame_count_max or `begin_write()` will panic.
	///
	/// It returns the number of frames you must actually write. The returned value
	/// will always be less than or equal to the provided value.
	///
	/// # Errors
	///
	/// * `Error::Invalid`
	///   * `frame_count` <= 0
	///   * `frame_count` < `frame_count_min` or `frame_count` > `frame_count_max`
	///   * function called too many times without respecting `frame_count_max`
	/// * `Error::Streaming`
	/// * `Error::Underflow` - an underflow caused this call to fail. You might
	///   also get an `underflow_callback()`, and you might not get
	///   this error code when an underflow occurs. Unlike `Error::Streaming`,
	///   the outstream is still in a valid state and streaming can continue.
	/// * `Error::IncompatibleDevice` - in rare cases it might just now
	///   be discovered that the device uses non-byte-aligned access, in which
	///   case this error code is returned.
	///
	pub fn begin_write(&mut self, frame_count: usize) -> Result<usize> {
		assert!(frame_count >= self.frame_count_min && frame_count <= self.frame_count_max, "frame_count out of range");

		let mut areas: *mut raw::SoundIoChannelArea = ptr::null_mut();
		let mut actual_frame_count: c_int = frame_count as _;

		match unsafe { raw::soundio_outstream_begin_write(self.outstream, &mut areas as *mut _, &mut actual_frame_count as *mut _) } {
			0 => {
				self.write_started = true;
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

	/// Commits the write that you began with `begin_write()`.
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
	pub fn end_write(&mut self) {
		if self.write_started {
			unsafe {
				match raw::soundio_outstream_end_write(self.outstream) {
					0 => {self.write_started = false;},
					x => println!("Error writing outstream: {}", Error::from(x)),
				}
			}
		}
	}
	
	/// Get the minimum frame count that you can call `begin_write()` with.
	/// Retreive this value before calling `begin_write()` to ensure you read the correct number
	/// of frames.
	pub fn frame_count_min(&self) -> usize {
		self.frame_count_min
	}

	/// Get the maximum frame count that you can call `begin_write()` with.
	/// Retreive this value before calling `begin_write()` to ensure you read the correct number
	/// of frames.
	pub fn frame_count_max(&self) -> usize {
		self.frame_count_max
	}

	/// Get the actual frame count that you did call `begin_write()` with. Panics if you haven't called
	/// `begin_write()` yet.
	pub fn frame_count(&self) -> usize {
		assert!(self.write_started);
		self.frame_count
	}

	/// Get latency due to software only, not including hardware.
	pub fn software_latency(&self) -> f64 {
		unsafe {
			(*self.outstream).software_latency as _
		}
	}

	/// Return the number of channels in this stream. Guaranteed to be at least 1.
	pub fn channel_count(&self) -> usize {
		unsafe {
			(*self.outstream).layout.channel_count as _
		}
	}

	/// Get the sample rate in Hertz.
	pub fn sample_rate(&self) -> i32 {
		unsafe {
			(*self.outstream).sample_rate as _
		}
	}

	/// Obtain the total number of seconds that the next frame written after the
	/// last frame written from the write callback will take to become
	/// audible. This includes both software and hardware latency. In other words,
	/// if you call this function directly after dropping the `OutStreamWriter`,
	/// this gives you the number of seconds that the next frame written will take
	/// to become audible.
	///
	/// # Errors
	///
	/// * `Error::Streaming`
	///
	pub fn get_latency(&mut self) -> Result<f64> {
		let mut x: c_double = 0.0;
		match unsafe { raw::soundio_outstream_get_latency(self.outstream, &mut x as *mut c_double) } {
			0 => Ok(x),
			e => Err(e.into()),
		}
	}

	/// Set the value of a sample/channel. This panics if the `channel` or `frame` are
	/// out of range or if you haven't called `begin_write()` yet.
	///
	/// If you use a different type from the actual one it will be converted.
	///
	/// # Examples
	///
	/// ```
	/// fn write_callback(stream: &mut soundio::OutStreamWriter) {
	///     let frame_count_max = stream.frame_count_max();
	///     stream.begin_write(frame_count_max).unwrap();
	///     for c in 0..stream.channel_count() {
	///         for f in 0..stream.frame_count() {
	///             stream.set_sample::<f32>(c, f, 0.0 as f32);
	///         }
	///     }
	/// }
	/// ```
	pub fn set_sample<T: Sample>(&mut self, channel: usize, frame: usize, sample: T) {
		assert!(self.write_started);

		assert!(channel < self.channel_count(), "Channel out of range");
		assert!(frame < self.frame_count(), "Frame out of range");

		unsafe {
			let ptr = self.channel_areas[channel].ptr.offset((frame * self.channel_areas[channel].step as usize) as isize) as *mut u8;

			match (*self.outstream).format {
				raw::SoundIoFormat::SoundIoFormatS8 => i8::to_raw_le(T::to_i8(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatU8 => u8::to_raw_le(T::to_u8(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatS16LE => i16::to_raw_le(T::to_i16(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatS16BE => i16::to_raw_be(T::to_i16(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatU16LE => u16::to_raw_le(T::to_u16(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatU16BE => u16::to_raw_be(T::to_u16(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatS24LE => i24::to_raw_le(T::to_i24(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatS24BE => i24::to_raw_be(T::to_i24(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatU24LE => u24::to_raw_le(T::to_u24(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatU24BE => u24::to_raw_be(T::to_u24(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatS32LE => i32::to_raw_le(T::to_i32(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatS32BE => i32::to_raw_be(T::to_i32(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatU32LE => u32::to_raw_le(T::to_u32(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatU32BE => u32::to_raw_be(T::to_u32(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatFloat32LE => f32::to_raw_le(T::to_f32(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatFloat32BE => f32::to_raw_be(T::to_f32(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatFloat64LE => f64::to_raw_le(T::to_f64(sample), ptr),
				raw::SoundIoFormat::SoundIoFormatFloat64BE => f64::to_raw_be(T::to_f64(sample), ptr),
				_ => panic!("Unknown format"),			
			}
		}
	}

	// TODO: To acheive speed *and* safety I can use iterators. That will be in a future API.
}

impl<'a> Drop for OutStreamWriter<'a> {
	/// This will drop all of the frames from when you called `begin_write()`.
	///
	/// Errors are currently are just printed to the console and ignored.
	///
	/// # Errors
	///
	/// * `Error::Streaming`
	fn drop(&mut self) {
		if self.write_started {
			unsafe {
				match raw::soundio_outstream_end_write(self.outstream) {
					0 => {},
					x => println!("Error writing outstream: {}", Error::from(x)),
				}
			}
		}
	}
}
