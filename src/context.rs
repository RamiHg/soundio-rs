extern crate libsoundio_sys as raw;

use super::device::*;
use super::error::*;
use super::backend::*;

use std::ptr;
use std::os::raw::{c_int, c_char};
use std::marker::PhantomData;

/// `Context` represents the libsoundio library context.
///
/// It must be created using `Context::new()` before most operations can be done and you
/// generally will only have one context object per app.
///
/// The underlying C struct is destroyed (and the backend is disconnected) when this object
/// is dropped, which means that it must outlive all the Devices it creates. This is
/// enforced via the lifetime system.
///
/// # Examples
///
/// ```
/// let mut ctx = soundio::Context::new();
/// ```
pub struct Context<'a> {
	/// The soundio library instance.
	soundio: *mut raw::SoundIo,
	/// The app name, used by some backends.
	app_name: String,
	/// The optional callbacks. They are boxed so that we can take a raw pointer
	/// to the heap object and give use it as `void* userdata`.
	userdata: Box<ContextUserData<'a>>,
}

// The callbacks required for a context are stored in this object.
pub struct ContextUserData<'a> {
	backend_disconnect_callback: Option<Box<FnMut(Error) + 'a>>,
	devices_change_callback: Option<Box<FnMut() + 'a>>,
	events_signal_callback: Option<Box<FnMut() + 'a>>,
}

// See `Context::new_with_callbacks()`.
extern fn on_backend_disconnect(sio: *mut raw::SoundIo, err: c_int) {
	let err = Error::from(err);
	
	// Use sio.userdata to get a reference to the ContextUserData object.
	let raw_userdata_pointer = unsafe { (*sio).userdata as *mut ContextUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	if let Some(ref mut cb) = userdata.backend_disconnect_callback {
		cb(err);
	} else {
		// Hmm I decided to replicate the libsoundio behaviour.
		panic!("Backend disconnected: {}", err);
	}
}

// See `Context::new_with_callbacks()`.
extern fn on_devices_change(sio: *mut raw::SoundIo) {
	// Use sio.userdata to get a reference to the ContextUserData object.
	let raw_userdata_pointer = unsafe { (*sio).userdata as *mut ContextUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	if let Some(ref mut cb) = userdata.devices_change_callback {
		cb();
	} else {
		println!("Devices changed");
	}
}

// See `Context::new_with_callbacks()`.
extern fn on_events_signal(sio: *mut raw::SoundIo) {
	// Use sio.userdata to get a reference to the ContextUserData object.
	let raw_userdata_pointer = unsafe { (*sio).userdata as *mut ContextUserData };
	let userdata = unsafe { &mut (*raw_userdata_pointer) };

	if let Some(ref mut cb) = userdata.events_signal_callback {
		cb();
	}
}

/// Optional real time priority warning callback.
///
/// This callback is fired when making thread real-time priority failed. The default
/// callback action prints a message instructing the user how to configure their
/// system to allow real-time priority threads. The message is printed to stderr
/// and is only printed once.
///
/// Because the callback doesn't take any `void* userdata` parameters
/// I can't see an easy way to integrate it into the Rust API, so I have chosen
/// to just use the default function. In otherwords the callback here is not used.
#[allow(dead_code)]
extern fn emit_rtprio_warning() {
}

impl<'a> Context<'a> {

	/// Create a new libsoundio context.
	///
	/// This panics if libsoundio fails to create the context object. This only happens due to out-of-memory conditions
	/// and Rust also panics (aborts actually) under those conditions in the standard library so this behaviour seemed acceptable.
	///
	/// You can create multiple `Context` instances to connect to multiple backends.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ```
	pub fn new() -> Context<'a> {
		let soundio = unsafe { raw::soundio_create() };
		if soundio == ptr::null_mut() {
			panic!("soundio_create() failed (out of memory).");
		}

		let mut context = Context { 
			soundio: soundio,
			// The default name in libsoundio is "SoundIo". We replicate that here for `Context::app_name()`.
			app_name: "SoundIo".to_string(),
			userdata: Box::new( ContextUserData {
				backend_disconnect_callback: None,
				devices_change_callback: None,
				events_signal_callback: None,
			})
		};

		// Note that libsoundio's default on_backend_disconnect() handler panics!
		// That may actually be reasonable behaviour. I'm not sure under which conditions
		// disconnects occur.
		unsafe {
			(*context.soundio).on_backend_disconnect = Some(on_backend_disconnect);
			(*context.soundio).on_devices_change = Some(on_devices_change);
			(*context.soundio).on_events_signal = Some(on_events_signal);
			// (*context.soundio).app_name is already set by default to point to a static C string "SoundIo".

			// This callback is not used - see its documentation for more information.
			// (*context.soundio).emit_rtprio_warning = emit_rtprio_warning as *mut _;

			// Save a reference here so that we can have user-defined callbacks.
			(*context.soundio).userdata = context.userdata.as_mut() as *mut ContextUserData as *mut _;
		}
		context
	}

	/// Create a new libsoundio context with some callbacks specified.
	///
	/// This is the same as `Context::new()` but allows you to specify the following optional callbacks.
	///
	/// ## `backend_disconnect_callback`
	///
	/// This is called when the backend disconnects. For example,
	/// when the JACK server shuts down. When this happens, listing devices
	/// and opening streams will always fail with
	/// `Error::BackendDisconnected`. This callback is only called during a
	/// call to `Context::flush_events()` or `Context::wait_events()`.
	/// If you do not supply a callback, the default will panic
	/// with an error message. This callback is also called when the thread
	/// that retrieves device information runs into an unrecoverable condition
	/// such as running out of memory.
	///
	/// The possible errors passed to the callback are:
	///
	/// * `Error::BackendDisconnected`
	/// * `Error::NoMem`
	/// * `Error::SystemResources`
	/// * `Error::ErrorOpeningDevice - unexpected problem accessing device information
	///
	/// ## `devices_change_callback`
	///
	/// This is called when the list of devices change. It is only called during a call
	/// to `Context::flush_events()` or `Context::`wait_events()`. The default behaviour is
	/// to print "Devices changed" to the console, which you can disable by using an empty callback.
	///
	/// ## `events_signal_callback`
	///
	/// This is called from an unknown thread that you should not use
	/// to call any soundio functions. You may use this to signal a condition
	/// variable to wake up. It is called when `Context::wait_events()` would be woken up.
	///
	/// # Examples
	///
	/// ```
	/// let backend_disconnect_callback = |err| { println!("Backend disconnected: {}", err); };
	///
	/// let mut ctx = soundio::Context::new_with_callbacks(
	///     Some(backend_disconnect_callback),
	///     None::<fn()>,
	///     None::<fn()>,
	/// );
	/// ```
	pub fn new_with_callbacks<BackendDisconnectCB, DevicesChangeCB, EventsSignalCB> (
				backend_disconnect_callback: Option<BackendDisconnectCB>,
				devices_change_callback: Option<DevicesChangeCB>,
				events_signal_callback: Option<EventsSignalCB>,
			) -> Context<'a>
		where
			BackendDisconnectCB: 'a + FnMut(Error),
			DevicesChangeCB: 'a + FnMut(),
			EventsSignalCB: 'a + FnMut() {
		// A set of function like `fn set_XX_callback(&mut self, ...` might be a nicer interface
		// but I am unsure about the safety implications.
		let mut context = Context::new();

		if let Some(cb) = backend_disconnect_callback {
			context.userdata.backend_disconnect_callback = Some(Box::new(cb));
		}
		if let Some(cb) = devices_change_callback {
			context.userdata.devices_change_callback = Some(Box::new(cb));
		}
		if let Some(cb) = events_signal_callback {
			context.userdata.events_signal_callback = Some(Box::new(cb));
		}

		context
	}

	/// Set the app name. This is shown in JACK and PulseAudio. Any colons are removed. The default is "SoundIo".
	///
	/// This must be called before you connect to a backend.
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.set_app_name("My App");
	/// ```
	pub fn set_app_name(&mut self, name: &str) {
		self.app_name = name.chars().filter(|&x| x != ':').collect();
		unsafe { (*self.soundio).app_name = self.app_name.as_ptr() as *mut c_char; }
	}

	/// Get the app name previously set by `set_app_name()`.
	/// The default is "SoundIo".
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// assert_eq!(ctx.app_name(), "SoundIo");
	/// ctx.set_app_name(":::My App:::");
	/// assert_eq!(ctx.app_name(), "My App");
	/// ```
	pub fn app_name(&self) -> String {
		self.app_name.clone()
	}

	/// Connect to the default backend, trying them in the order returned by `available_backends()`.
	/// It will fail with `Error::Invalid` if this instance is already connected to a backend.
	///
	/// # Return Values
	///
	/// * `soundio::Error::Invalid` if you are already connected.
	/// * `soundio::Error::NoMem`
	/// * `soundio::Error::SystemResources`
	/// * `soundio::Error::NoSuchClient` when JACK returns `JackNoSuchClient`.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// match ctx.connect() {
	/// 	Ok(()) => println!("Connected to {}", ctx.current_backend()),
	/// 	Err(e) => println!("Couldn't connect: {}", e),
	/// }
	/// ```
	pub fn connect(&mut self) -> Result<()> {
		let ret = unsafe { raw::soundio_connect(self.soundio) };
		match ret {
			0 => Ok(()),
			_ => Err(ret.into()),
		}
	}

	/// Connect to the specified backend. It will fail with `Error::Invalid` if this instance
	/// is already connected to a backend.
	///
	/// # Return Values
	///
	/// * `soundio::Error::Invalid` if you are already connected or the backend was invalid.
	/// * `soundio::Error::NoMem`
	/// * `soundio::Error::BackendUnavailable` if the backend was not compiled in.
	/// * `soundio::Error::SystemResources`
	/// * `soundio::Error::NoSuchClient` when JACK returns `JackNoSuchClient`.
	/// * `soundio::Error::InitAudioBackend` if the requested backend is not active.
	/// * `soundio::Error::BackendDisconnected` if the backend disconnected while connecting. See also [bug 103](https://github.com/andrewrk/libsoundio/issues/103)
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// match ctx.connect_backend(soundio::Backend::Dummy) {
	/// 	Ok(()) => println!("Connected to dummy backend"),
	/// 	Err(e) => println!("Couldn't connect: {}", e),
	/// }
	/// ```
	pub fn connect_backend(&mut self, backend: Backend) -> Result<()> {
		let ret = unsafe { raw::soundio_connect_backend(self.soundio, backend.into()) };
		match ret {
			0 => Ok(()),
			_ => Err(ret.into()),
		}
	}

	/// Disconnect from the current backend. Does nothing if no backend is connected.
	/// It is usually not necessary to call this manually; the backend will disconnect
	/// automatically when `Context` is dropped.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// match ctx.connect() {
	/// 	Ok(()) => println!("Connected to {}", ctx.current_backend()),
	/// 	Err(e) => { println!("Couldn't connect: {}", e); return; },
	/// }
	/// ctx.disconnect();
	/// ```
	pub fn disconnect(&mut self) {
		unsafe {
			raw::soundio_disconnect(self.soundio);
		}
	}

	/// Return the current `Backend`.
	///
	/// If this `Context` isn't connected to any backend it returns `Backend::None`.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// match ctx.connect() {
	/// 	Ok(()) => println!("Connected to {}", ctx.current_backend()),
	/// 	Err(e) => println!("Couldn't connect: {}", e),
	/// }
	/// ```
	pub fn current_backend(&self) -> Backend {
		unsafe {
			(*self.soundio).current_backend.into()
		}
	}

	/// Return a list of available backends on this system.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// println!("Available backends: {:?}", ctx.available_backends());
	/// ```
	pub fn available_backends(&self) -> Vec<Backend> {
		let count = unsafe { raw::soundio_backend_count(self.soundio) };
		let mut backends = Vec::with_capacity(count as usize);
		for i in 0..count {
			backends.push( unsafe { raw::soundio_get_backend(self.soundio, i).into() } );
		}
		backends
	}

	/// Atomically update information for all connected devices. Note that calling
	/// this function merely flips a pointer; the actual work of collecting device
	/// information is done elsewhere. It is performant to call this function many
	/// times per second.
	///
	/// When you call this, the following callbacks might be called:
	/// 
	/// * `on_devices_change`
	/// * `on_backend_disconnect`
	///
	/// The callbacks are specified in `Context::new_with_callbacks()`. This is the
	/// only time those callbacks can be called.
	///
	/// This must be called from the same thread as the thread in which you call
	/// any function that gets an input or output device, count or index (e.g.
	/// `Context::default_input_device_index()`).
	///
	/// Note that if you do not care about learning about updated devices, you
	/// can call this function only once ever and never call `Context::wait_events()`.
	pub fn flush_events(&self) {
		unsafe {
			raw::soundio_flush_events(self.soundio);
		}
	}

	/// This function calls `Context::flush_events()` then blocks until another event
	/// is ready or you call `wakeup`. Be ready for spurious wakeups.
	pub fn wait_events(&self) {
		unsafe {
			raw::soundio_wait_events(self.soundio);
		}
	}

	/// Wake up any other threads currently blocking in `Context::wait_events()`.
	///
	/// In order to enable this functionality, `Context` implements the `Send` and
	/// `Sync` traits despite the fact that not all functions can be called from all threads.
	/// Be careful.
	pub fn wakeup(&self) {
		
		// To do this properly it might be necessary to split Context into multiple objects, one for
		// each thread, or maybe one that is Send/Sync and another that isn't. That is rather
		// complicated however and there is probably a better way.
		unsafe {
			raw::soundio_wakeup(self.soundio);
		}
	}

	/// If necessary you can manually trigger a device rescan. Normally you will
	/// not ever have to call this function, as libsoundio listens to system events
	/// for device changes and responds to them by rescanning devices and preparing
	/// the new device information for you to be atomically replaced when you call
	/// `Context::flush_events()`. However you might run into cases where you want to
	/// force trigger a device rescan, for example if an ALSA device has a probe error.
	///
	/// After you call this you still have to use `Context::flush_events()` or
	/// `Context::wait_events()` and then wait for the
	/// `devices_change_callback` to be called.
	///
	/// This can be called from any thread context except for the read or write callbacks.
	pub fn force_device_scan(&self) {
		unsafe {
			raw::soundio_force_device_scan(self.soundio);
		}
	}

	/// Use this function to retrieve an input device given its index. Before getting devices
	/// you must call `Context::flush_events()` at least once, otherwise this will return
	/// an error. It will also return an error if there is a probe error while opening the
	/// device or the index is out of bounds (use `Context::input_device_count()` to learn)
	/// how many input devices there are.
	///
	/// It is probably more convenient to use the `Context::input_devices()` function instead
	/// of this one unless you have some very specific requirements.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// for i in 0..ctx.input_device_count() {
	///     let dev = ctx.input_device(i).expect("Error opening device");
	///     println!("Device {} is called {}", i, dev.name());
	/// }
	/// ```
	pub fn input_device(&self, index: usize) -> Result<Device> {
		let device = unsafe { raw::soundio_get_input_device(self.soundio, index as c_int) };
		if device == ptr::null_mut() {
			return Err(Error::OpeningDevice);
		}

		let probe_error = unsafe { (*device).probe_error };

		if probe_error != 0 {
			return Err(probe_error.into());
		}

		Ok(Device {
			device: device,
			phantom: PhantomData,
		})
	}

	/// Use this function to retrieve an output device given its index. Before getting devices
	/// you must call `Context::flush_events()` at least once, otherwise this will return
	/// an error. It will also return an error if there is a probe error while opening the
	/// device or the index is out of bounds (use `Context::output_device_count()` to learn)
	/// how many output devices there are.
	///
	/// It is probably more convenient to use the `Context::output_devices()` function instead
	/// of this one unless you have some very specific requirements.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// for i in 0..ctx.output_device_count() {
	///     let dev = ctx.output_device(i).expect("Error opening device");
	///     println!("Device {} is called {}", i, dev.name());
	/// }
	/// ```
	pub fn output_device(&self, index: usize) -> Result<Device> {
		let device = unsafe { raw::soundio_get_output_device(self.soundio, index as c_int) };
		if device == ptr::null_mut() {
			return Err(Error::OpeningDevice);
		}

		let probe_error = unsafe { (*device).probe_error };

		if probe_error != 0 {
			return Err(probe_error.into());
		}

		Ok(Device {
			device: device,
			phantom: PhantomData,
		})
	}

	/// Get the number of input devices in this machine. You *must* call
	/// `Context::flush_events()` at least once before calling this function
	/// otherwise it will panic!
	pub fn input_device_count(&self) -> usize {
		let count = unsafe { raw::soundio_input_device_count(self.soundio) };
		assert!(count != -1, "flush_events() must be called before input_device_count()");
		count as _
	}

	/// Get the number of output devices in this machine. You *must* call
	/// `Context::flush_events()` at least once before calling this function
	/// otherwise it will panic!
	pub fn output_device_count(&self) -> usize {
		let count = unsafe { raw::soundio_output_device_count(self.soundio) };
		assert!(count != -1, "flush_events() must be called before output_device_count()");
		count as _
	}
	
	/// Returns the index of the default input device. You must call
	/// `Context::flush_events()` at least once before calling this function.
	/// If there are no input devices, or you never called `flush_events()` it
	/// returns `None`
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// let default_input = ctx.default_input_device_index();
	/// for i in 0..ctx.input_device_count() {
	///     let dev = ctx.input_device(i).expect("Error opening device");
	///     println!("Device {} is called {}", i, dev.name());
	///     if Some(i) == default_input {
	///         println!("And it's the default!");
	///     }
	/// }
	/// ```
	pub fn default_input_device_index(&self) -> Option<usize> {
		let index = unsafe { raw::soundio_default_input_device_index(self.soundio) };
		match index {
			-1 => None,
			_ => Some(index as usize),
		}
	}

	/// Returns the index of the default output device. You must call
	/// `Context::flush_events()` at least once before calling this function.
	/// If there are no output devices, or you never called `flush_events()` it
	/// returns `None`
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// let default_output = ctx.default_output_device_index();
	/// for i in 0..ctx.output_device_count() {
	///     let dev = ctx.output_device(i).expect("Error opening device");
	///     println!("Device {} is called {}", i, dev.name());
	///     if Some(i) == default_output {
	///         println!("And it's the default!");
	///     }
	/// }
	/// ```
	pub fn default_output_device_index(&self) -> Option<usize> {
		let index = unsafe { raw::soundio_default_output_device_index(self.soundio) };
		match index {
			-1 => None,
			_ => Some(index as usize),
		}
	}

	/// Get all the input devices as a vector. You *must* call `Context::flush_events()`
	/// at least once before calling this function. If you don't it will panic.
	/// 
	/// It returns an error if there is an error opening any of the devices.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// let devs = ctx.input_devices().expect("Error getting devices");
	/// for dev in devs {
	///     println!("Device {} ", dev.name());
	/// }
	/// ```
	pub fn input_devices(&self) -> Result<Vec<Device>> {
		let count = self.input_device_count();
		let mut devices = Vec::new();
		for i in 0..count {
			devices.push(self.input_device(i)?);
		}
		Ok(devices)
	}

	/// Get all the output devices as a vector. You *must* call `Context::flush_events()`
	/// at least once before calling this function. If you don't it will panic.
	/// 
	/// It returns an error if there is an error opening any of the devices.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// let devs = ctx.output_devices().expect("Error getting devices");
	/// for dev in devs {
	///     println!("Device {} ", dev.name());
	/// }
	/// ```
	pub fn output_devices(&self) -> Result<Vec<Device>> {
		let count = self.output_device_count();
		let mut devices = Vec::new();
		for i in 0..count {
			devices.push(self.output_device(i)?);
		}
		Ok(devices)
	}

	/// Get the default input device. You *must* call `Context::flush_events()`
	/// at least once before calling this function. If you don't it will panic.
	///
	/// If there are no devices it returns `Error::NoSuchDevice`. If there was
	/// an error opening the device it returns that error.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// let dev = ctx.default_input_device().expect("No default device");
	/// println!("The default input device is {}", dev.name());
	/// ```
	pub fn default_input_device(&self) -> Result<Device> {
		let index = match self.default_input_device_index() {
			Some(x) => x,
			None => return Err(Error::NoSuchDevice),
		};
		self.input_device(index)
	}
	
	/// Get the default output device. You *must* call `Context::flush_events()`
	/// at least once before calling this function. If you don't it will panic.
	///
	/// If there are no devices it returns `Error::NoSuchDevice`. If there was
	/// an error opening the device it returns that error.
	///
	/// # Examples
	///
	/// ```
	/// let mut ctx = soundio::Context::new();
	/// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
	/// ctx.flush_events();
	/// let dev = ctx.default_output_device().expect("No default device");
	/// println!("The default output device is {}", dev.name());
	/// ```
	pub fn default_output_device(&self) -> Result<Device> {
		let index = match self.default_output_device_index() {
			Some(x) => x,
			None => return Err(Error::NoSuchDevice),
		};
		self.output_device(index)
	}
}

impl<'a> Drop for Context<'a> {
	fn drop(&mut self) {
		unsafe {
			// This also disconnects if necessary.
			raw::soundio_destroy(self.soundio);
		}
	}
}

// This allows wakeup and wait_events to be called from other threads.
// TODO: Find out exactly the thread-safety properties of libsoundio.
unsafe impl<'a> Send for Context<'a> {}
unsafe impl<'a> Sync for Context<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::backend::*;

    #[test]
    fn connect_default_backend() {
		let mut ctx = Context::new();
		match ctx.connect_backend(Backend::Dummy) {
			Ok(()) => println!("Connected to {}", ctx.current_backend()),
			Err(e) => println!("Couldn't connect: {}", e),
		}
    }

	#[test]
	fn available_backends() {
		let ctx = Context::new();
		println!("Available backends: {:?}", ctx.available_backends());
	}

	// TODO: More tests.
}