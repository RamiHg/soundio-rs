//! # soundio-rs
//!
//! The soundio-rs crate is a wrapper for [libsoundio](http://libsound.io/).
//! 
//! The API closely follows the libsoundio so it is fairly low level but much safer.
//! Most of the libsoundio API is exposed.
//!
//! Some examples are included that are roughly equivalent to the examples in libsoundio.
//!
//! # Basic Usage
//!
//! First you must create a new instance of the library using `Context::new()` as follows.
//!
//! ```
//! let ctx = soundio::Context::new();
//! ```
//!
//! This will never fail except for out-of-memory situations in which case it panics (this is
//! standard Rust behaviour).
//!
//! Next you can connect to a backend. You can specify the backend explicitly, but the simplest
//! thing is to leave it unspecified, in which case they are all tried in order. You can also
//! set the name of the app if you like.
//!
//! ```rust,ignore
//! ctx.set_app_name("Player");
//! ctx.connect()?;
//! ```
//!
//! Assuming that worked ok, you can now find a device (or devices) to play or record from.
//! However before you can open any devices you must flush events like this.
//!
//! ```
//! # let mut ctx = soundio::Context::new();
//! # ctx.connect_backend(soundio::Backend::Dummy).unwrap();
//! ctx.flush_events();
//! ```
//!
//! The simplest way to open a device is to just open the default input or output device as follows.
//!
//! ```
//! # let mut ctx = soundio::Context::new();
//! # ctx.connect_backend(soundio::Backend::Dummy).unwrap();
//! let dev = ctx.default_input_device().expect("No input device");
//! ```
//! 
//! However *please* don't only use that option. Your users will hate you when they have to work out
//! how ALSA's undocumented and convoluted `.asoundrc` config systems works just to have your app use
//! a different sound card.
//! 
//! To let the user select the output device you can make use of `Context::input_devices()` and `Context::output_devices()`.
//!
//! Onces the device has been opened, you can query it for supported formats and sample rates.
//!
//! ```
//! # fn foo() -> Result<(), String> {
//! # let mut ctx = soundio::Context::new();
//! # ctx.connect_backend(soundio::Backend::Dummy).unwrap();
//! # let dev = ctx.default_input_device()?;
//! #
//! if !dev.supports_layout(soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Stereo)) {
//!     return Err("Device doesn't support stereo".to_string());
//! }
//! if !dev.supports_format(soundio::Format::S16LE) {
//!     return Err("Device doesn't support S16LE".to_string());
//! }
//! if !dev.supports_sample_rate(44100) {
//!     return Err("Device doesn't 44.1 kHz".to_string());
//! }
//! #
//! # Ok(())
//! # }
//! ```
//!
//! If all is well we can open an input or output stream. You can only open an input stream on an input
//! device, and an output stream on an output device. If a physical device supports input and output it
//! is split into two `Device` instances, with different `Device::aim()`s but the same `Device::id()`.
//!
//! To open the stream you need to define some callbacks for reading/writing to it. The only required one
//! is the read/write callback. You also need to specify the latency in seconds, which determines how often
//! your callback is called.
//!
//! ```
//! # fn foo() -> Result<(), String> {
//! # let mut ctx = soundio::Context::new();
//! # ctx.connect_backend(soundio::Backend::Dummy).unwrap();
//! # let dev = ctx.default_input_device()?;
//! #
//! let mut input_stream = dev.open_instream(
//!     44100,
//!     soundio::Format::S16LE,
//!     soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Stereo),
//!     2.0,
//!     read_callback,
//!     None::<fn()>,
//!     None::<fn(soundio::Error)>,
//! )?;
//! #
//! # Ok(())
//! # }
//! #
//! # fn read_callback(stream: &mut soundio::InStreamReader) { }
//! ```
//!
//! `read_callback` is a callback that takes an `InStreamReader` or `OutStreamWriter`, something like this.
//!
//! ```
//! fn read_callback(stream: &mut soundio::InStreamReader) {
//!     let frame_count_max = stream.frame_count_max();
//!     if let Err(e) = stream.begin_read(frame_count_max) {
//!         println!("Error reading from stream: {}", e);
//!         return;
//!     }
//!    
//!     for f in 0..stream.frame_count() {
//!         for c in 0..stream.channel_count() {
//!             do_something_with(stream.sample::<i16>(c, f));
//!         }
//!     }
//! }
//! # fn do_something_with(_: i16) { }
//! ```
//!
//! In memory samples are stored LRLRLRLR rather than LLLLRRRR so for optimisation purposes it is
//! probably better to loop over frames and then channels, rather than the other way around (though I've
//! not tested the actual effect this has).
//!
//! Finally call `InStream::start()` to start your stream.
//!
//! ```rust,ignore
//! input_stream.start()?;
//! ```
//!
//! There are some extra details regarding `Context::wait_events()` and `Context::wakeup()`, and you
//! will likely want to use scoped threads via the `crossbeam` crate for those. The best way to learn
//! more is to see the examples.
//!
//! # Examples
//!
//! ## list_devices
//! 
//! This example is very similar to libsoundio's list_devices example. It simply lists the devices
//! on the system. It currently has no command line options.
//!
//! ## recorder
//!
//! This records audio to a wav file until you press enter. Note that it actually writes the wav
//! file in the audio callback which is a bad idea because writing files can be slow. In a real
//! program it might be better to have a separate thread for buffered file writing.
//!
//! ## player
//!
//! The opposite of recorder - it plays a wav file. This also has the flaw of reading the file in 
//! the audio callback. Also currently it does not exit when the file ends because I am still learning Rust.
//!
//! ## sine
//!
//! A very simple example that plays a sine wave.
//!
//! # Bugs, Credits, etc.
//!
//! libsoundio was written by Andrew Kelley (legend). This wrapper was written by Tim Hutt. There is
//! another wrapper available [here](https://github.com/klingtnet/rsoundio) if this one doesn't
//! satisfy you for some reason. It is developed [on github](https://github.com/Timmmm/soundio-rs).
//! Bugs, suggestions and praise are welcome!

extern crate libsoundio_sys as raw;

mod types;
mod context;
mod device;
mod instream;
mod outstream;
mod util;
mod layout;
mod error;
mod channels;
mod backend;
mod format;
mod sample;

pub use self::types::*;
pub use self::context::*;
pub use self::device::*;
pub use self::instream::*;
pub use self::outstream::*;
pub use self::layout::*;
pub use self::error::*;
pub use self::channels::*;
pub use self::backend::*;
pub use self::format::*;
pub use self::sample::*;

use self::util::*;

/// Return the libsoundio version string, for example `"1.0.2"`.
///
/// # Examples
///
/// ```
/// println!("libsoundio version: {}", soundio::version_string());
/// ```
pub fn version_string() -> String {
	latin1_to_string( unsafe { raw::soundio_version_string() } )
}

/// Return the libsoundio version as a tuple, for exaample `(1, 0, 2)`.
///
/// # Examples
///
/// ```
/// let version = soundio::version();
/// if version.0 == 1 && version.1 == 1 {
/// 	println!("Congrats! You are using libsoundio 1.1.x");
/// }
/// ```
pub fn version() -> (i32, i32, i32) {
	unsafe {
		(
			raw::soundio_version_major() as i32,
			raw::soundio_version_minor() as i32,
			raw::soundio_version_patch() as i32,
		)
	}
}

/// Return `true` if libsoundio supports the given `Backend`.
///
/// Although the internal implementation is slightly different, this is effectively
/// the same as using `Context::available_backends()` to check for backend support.
///
/// Both functions only check whether libsoundio was built with support for the
/// given backends. They don't try to connect to them or check they are supported
/// by the host system.
///
/// # Examples
///
/// ```
/// let backend_list = [
/// 	soundio::Backend::Jack,
/// 	soundio::Backend::PulseAudio,
/// 	soundio::Backend::Alsa,
/// 	soundio::Backend::CoreAudio,
/// 	soundio::Backend::Wasapi,
/// 	soundio::Backend::Dummy,
/// ];
///
/// for &backend in backend_list.iter() {
/// 	println!("Backend {} available? {}", backend, soundio::have_backend(backend));
/// } 
/// ```
pub fn have_backend(backend: Backend) -> bool {
	unsafe {
		raw::soundio_have_backend(backend.into()) != 0
	}
}

