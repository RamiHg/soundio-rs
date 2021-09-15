extern crate libsoundio_sys as raw;

use super::error::*;
use super::format::*;
use super::instream::*;
use super::layout::*;
use super::outstream::*;
use super::types::*;
use super::util::*;

use std::fmt::Display;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::slice;

/// Device represents an input or output device.
///
/// It is obtained from a `Context` using `Context::input_device()` or `Context::output_device()`.
/// You can use it to open an input stream or output stream.
#[derive(Debug)]
pub struct Device<'a> {
    /// The raw pointer to the device.
    pub device: *mut raw::SoundIoDevice,

    /// This is just here to say that Device cannot outlive the Context it was created from.
    /// 'a is the lifetime of that Context.
    pub phantom: PhantomData<&'a ()>,
}

impl<'a> Device<'a> {
    /// A string that uniquely identifies this device.
    ///
    /// If the same physical device supports both input and output, it is split
    /// into one `Device` for the input and another for the output.
    ///
    /// In this case, the `id` of each `Device` will be the same, and
    /// `Device::aim()` will be different. Additionally, if the device
    /// supports raw mode, there may be up to four devices with the same `id`:
    /// one for each value of `Device::is_raw()` and one for each value of
    /// `Device::aim()`.
    pub fn id(&self) -> String {
        // This is not explicitly latin1 but it is described as 'a string of bytes' so
        // it may contain invalid UTF-8 sequences.
        latin1_to_string(unsafe { (*self.device).id })
    }

    /// User-friendly UTF-8 encoded text to describe the device.
    pub fn name(&self) -> String {
        // This is explicitly UTF-8.
        utf8_to_string(unsafe { (*self.device).name })
    }

    /// Tells whether this device is an input device or an output device.
    ///
    /// If a physical device supports input and output it is split into two
    /// `Device`s, with the same `Device::id()` but different `Device::aim()`s.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = soundio::Context::new();
    /// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
    /// for dev in ctx.input_devices().expect("Couldn't get input devices") {
    ///     assert_eq!(dev.aim(), soundio::DeviceAim::Input);
    /// }
    /// for dev in ctx.output_devices().expect("Couldn't get output devices") {
    ///     assert_eq!(dev.aim(), soundio::DeviceAim::Output);
    /// }
    /// ```
    pub fn aim(&self) -> DeviceAim {
        unsafe { (*self.device).aim.into() }
    }

    /// Returns the list of channel layouts supported by this device.
    /// A channel layout has a name, and a list of channels with a channel ID.
    /// For examples `ChannelLayout { name: "Stereo", channels: vec![ChannelId::Left, ChannelId::Right] }`.
    ///
    /// Devices are guaranteed to have at least 1 channel layout.
    ///
    /// If you call `sort_channel_layouts()` before this function, the layouts will
    /// be sorted by the number of channels in decreasing order.
    pub fn layouts(&self) -> Vec<ChannelLayout> {
        let layouts_slice = unsafe {
            slice::from_raw_parts::<raw::SoundIoChannelLayout>(
                (*self.device).layouts,
                (*self.device).layout_count as _,
            )
        };

        layouts_slice.iter().map(|&x| x.into()).collect()
    }

    /// Get the current channel layout. This behaves similarly to the current format
    /// - this value is only meaningful for raw devices that have a sample
    /// rate defined before a stream is opened. See `Device::current_format()` for
    /// more information.
    pub fn current_layout(&self) -> ChannelLayout {
        unsafe { (*self.device).current_layout.into() }
    }

    /// List of formats this device supports.
    ///
    /// Devices are guaranteed to support at least one format.
    pub fn formats(&self) -> Vec<Format> {
        let formats_slice = unsafe {
            slice::from_raw_parts::<raw::SoundIoFormat>(
                (*self.device).formats,
                (*self.device).format_count as _,
            )
        };

        formats_slice.iter().map(|&x| x.into()).collect()
    }

    /// Get the current format.
    ///
    /// A device is either a raw device or it is a virtual device that is
    /// provided by a software mixing service such as dmix or PulseAudio (see
    /// `Device::is_raw()`). If it is a raw device, `current_format()` is meaningless;
    /// the device has no current format until you open it. On the other hand,
    /// if it is a virtual device, `current_format()` describes the
    /// destination sample format that your audio will be converted to. Or,
    /// if you're the lucky first application to open the device, you might
    /// cause the `current_format()` to change to your format.
    /// Generally, you want to ignore `current_format()` and use
    /// whatever format is most convenient for you which is supported by the device,
    /// because when you are the only application left, the mixer might decide to switch
    /// `current_format()` to yours. You can learn the supported formats via
    /// `Device::formats()`.
    ///
    /// If `current_format()` is unavailable, it will be set to `Format::Invalid`.
    pub fn current_format(&self) -> Format {
        unsafe { (*self.device).current_format.into() }
    }

    /// Sample rate is the number of frames per second (a frame is one sample from all channels).
    /// Sample rate is handled very similar to `formats()`.
    ///
    /// Devices are guaranteed to have at least 1 sample rate available.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = soundio::Context::new();
    /// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
    /// let out_dev = ctx.default_output_device().expect("Couldn't open default output");
    /// for rate in out_dev.sample_rates() {
    ///     println!("Sample rate min: {} max {}", rate.min, rate.max);
    /// }
    /// ```
    pub fn sample_rates(&self) -> Vec<SampleRateRange> {
        let sample_rates_slice = unsafe {
            slice::from_raw_parts::<raw::SoundIoSampleRateRange>(
                (*self.device).sample_rates,
                (*self.device).sample_rate_count as _,
            )
        };

        sample_rates_slice.iter().map(|&x| x.into()).collect()
    }

    /// Get the current sample rate. This behaves similarly to the current format
    /// - this value is only meaningful for raw devices that have a sample
    /// rate defined before a stream is opened. See `Device::current_format()` for
    /// more information.
    ///
    /// If `current_sample_rate()` is unavailable it will return 0.
    pub fn current_sample_rate(&self) -> i32 {
        unsafe { (*self.device).sample_rate_current as _ }
    }

    /// Software latency (current, minimum, maximum) in seconds. If this value is unknown or
    /// irrelevant, it is set to 0.0.
    ///
    /// For PulseAudio and WASAPI this value is unknown until you open a stream.
    pub fn software_latency(&self) -> SoftwareLatency {
        unsafe {
            SoftwareLatency {
                min: (*self.device).software_latency_min,
                max: (*self.device).software_latency_max,
                current: (*self.device).software_latency_current,
            }
        }
    }

    /// Return whether the device has raw access.
    ///
    /// Raw means that you are directly opening the hardware device and not
    /// going through a proxy such as dmix, PulseAudio, or JACK. When you open a
    /// raw device, other applications on the computer are not able to
    /// simultaneously access the device. Raw devices do not perform automatic
    /// resampling and thus tend to have fewer formats available.
    ///
    /// Physical devices will often have a raw `Device` and a virtual one. If the
    /// device supports input and output you will get four `Device`s.
    pub fn is_raw(&self) -> bool {
        unsafe { (*self.device).is_raw != 0 }
    }

    /// Sorts the channels returned by `layouts()` by channel count, descending.
    ///
    /// This mutates the internal list of layouts, which is why it takes `&mut self`.
    pub fn sort_channel_layouts(&mut self) {
        // It may be a good idea to remove this function. I don't think it adds to the API.
        unsafe {
            raw::soundio_device_sort_channel_layouts(self.device);
        }
    }

    /// Returns whether or not a given sample `Format` is supported by this device.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = soundio::Context::new();
    /// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
    /// let out_dev = ctx.default_output_device().expect("Couldn't open default output");
    /// println!("Default output device {} unsigned 16 bit little endian", if out_dev.supports_format(soundio::Format::S16LE) { "supports" } else { "doesn't support" });
    /// ```
    pub fn supports_format(&self, format: Format) -> bool {
        unsafe { raw::soundio_device_supports_format(self.device, format.into()) != 0 }
    }

    /// Returns whether or not a given channel layout is supported by this device.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = soundio::Context::new();
    /// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
    /// let out_dev = ctx.default_output_device().expect("Couldn't open default output");
    /// println!("Default output device {} stereo", if out_dev.supports_layout(soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Stereo)) { "supports" } else { "doesn't support" });
    /// ```
    pub fn supports_layout(&self, layout: ChannelLayout) -> bool {
        unsafe { raw::soundio_device_supports_layout(self.device, &layout.into() as *const _) != 0 }
    }

    /// Returns true if the given sample rate is supported by this device.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = soundio::Context::new();
    /// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
    /// let out_dev = ctx.default_output_device().expect("Couldn't open default output");
    /// println!("Default output device {} 44.1 kHz", if out_dev.supports_sample_rate(44100) { "supports" } else { "doesn't support" });
    /// ```
    pub fn supports_sample_rate(&self, sample_rate: i32) -> bool {
        unsafe { raw::soundio_device_supports_sample_rate(self.device, sample_rate as c_int) != 0 }
    }

    /// Returns the nearest supported sample rate of this device. Devices are guaranteed
    /// to support at least one sample rate.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = soundio::Context::new();
    /// ctx.connect_backend(soundio::Backend::Dummy).expect("Couldn't connect to backend");
    /// let out_dev = ctx.default_output_device().expect("Couldn't open default output");
    /// println!("Nearest sample rate to 44000: {}", out_dev.nearest_sample_rate(44000));
    /// ```
    pub fn nearest_sample_rate(&self, sample_rate: i32) -> i32 {
        unsafe { raw::soundio_device_nearest_sample_rate(self.device, sample_rate as c_int) as i32 }
    }

    /// After you call this function, SoundIoOutStream::software_latency is set to
    /// the correct value.
    ///
    /// The next thing to do is call ::soundio_outstream_start.
    /// If this function returns an error, the outstream is in an invalid state and
    /// you must call ::soundio_outstream_destroy on it.
    ///

    /// Open an output stream on an output device. After opening you can start, pause and stop it
    /// using the functions on the `OutStream` that is returned. Then your write callback
    /// will be called. See the documentation on `OutStreamWriter` for more information.
    ///
    /// The parameters are as follows.
    ///
    /// * `sample_rate` - The requested sample rate. Check supported sample rates first with `Device::sample_rates()`.
    /// * `format` - The requested format. Check supported formats first with `Device::formats()`.
    /// * `layout` - The requested channel layout. Check supported formats first with `Device::layouts()`.
    /// * `latency` - The requested software latency in seconds. With a lower value your write callback will be called more often and work in smaller blocks but latency will be lower.
    /// * `write_callback` - Required callback that is called to allow you to write audio data to the outstream. See `OutStreamWriter` for more details.
    /// * `underflow_callback` - Optional callback that is called when your `write_callback` is too slow and the output skips.
    /// * `error_callback` - Optional error callback.
    ///
    /// Currently it is not possible to set the outstream name, or libsoundio's `non_terminal_hint`.
    ///
    /// # Return Values
    ///
    /// If successful the function returns an `OutStream` which you can call `OutStream::start()` on,
    /// otherwise it returns one of the following errors:
    ///
    /// * `Error::Invalid`
    ///   - `aim()` is not `DeviceAim::Output`
    ///   - `format` is not valid
    ///   - `channel_count` is greater than `SOUNDIO_MAX_CHANNELS` (24).
    /// * `Error::NoMem`
    /// * `Error::OpeningDevice`
    /// * `Error::BackendDisconnected`
    /// * `Error::SystemResources`
    /// * `Error::NoSuchClient` - when JACK returns `JackNoSuchClient`
    /// * `Error::IncompatibleBackend` - `OutStream::channel_count()` is greater than the number of channels the backend can handle.
    /// * `Error::IncompatibleDevice` - stream parameters requested are not compatible with the chosen device.
    ///
    /// # Lifetimes
    ///
    /// `'a` is the lifetime of the `Device`. The `OutStream` lifetime `'b` must be less than or equal to `'a` (indicated by `'b: 'a`).
    /// Also the callbacks must have a lifetime greater than or equal to `'b`. They do not need to be `'static`.
    pub fn open_outstream<'b: 'a, WriteCB, UnderflowCB, ErrorCB>(
        &'a self,
        sample_rate: i32,
        format: Format,
        layout: ChannelLayout,
        latency: f64,
        write_callback: WriteCB,
        underflow_callback: Option<UnderflowCB>,
        error_callback: Option<ErrorCB>,
    ) -> Result<OutStream<'b>>
    where
        WriteCB: 'b + FnMut(&mut OutStreamWriter),
        UnderflowCB: 'b + FnMut(),
        ErrorCB: 'b + FnMut(Error),
    {
        let mut outstream = unsafe { raw::soundio_outstream_create(self.device) };
        if outstream.is_null() {
            // Note that we should really abort() here (that's what the rest of Rust
            // does on OOM), but there is no stable way to abort in Rust that I can see.
            panic!("soundio_outstream_create() failed (out of memory).");
        }

        unsafe {
            (*outstream).sample_rate = sample_rate;
            (*outstream).format = format.into();
            (*outstream).layout = layout.into();
            (*outstream).software_latency = latency;
            (*outstream).write_callback = outstream_write_callback;
            (*outstream).underflow_callback = Some(outstream_underflow_callback);
            (*outstream).error_callback = Some(outstream_error_callback);
        }

        let mut stream = OutStream {
            userdata: Box::new(OutStreamUserData {
                outstream,
                write_callback: Box::new(write_callback),
                underflow_callback: match underflow_callback {
                    Some(cb) => Some(Box::new(cb)),
                    None => None,
                },
                error_callback: match error_callback {
                    Some(cb) => Some(Box::new(cb)),
                    None => None,
                },
            }),
            phantom: PhantomData,
        };

        // Safe userdata pointer.
        unsafe {
            (*stream.userdata.outstream).userdata =
                stream.userdata.as_mut() as *mut OutStreamUserData as *mut _;
        }

        match unsafe { raw::soundio_outstream_open(stream.userdata.outstream) } {
            0 => {}
            x => return Err(x.into()),
        };

        match unsafe { (*stream.userdata.outstream).layout_error } {
            0 => {}
            x => return Err(x.into()),
        }

        Ok(stream)
    }

    /// Open an input stream on an input device. After opening you can start, pause and stop it
    /// using the functions on the `InStream` that is returned. Then your read callback
    /// will be called. See the documentation on `InStreamReader` for more information.
    ///
    /// The parameters are as follows.
    ///
    /// * `sample_rate` - The requested sample rate. Check supported sample rates first with `Device::sample_rates()`.
    /// * `format` - The requested format. Check supported formats first with `Device::formats()`.
    /// * `layout` - The requested channel layout. Check supported formats first with `Device::layouts()`.
    /// * `latency` - The requested software latency in seconds. With a lower value your read callback will be called more often and work in smaller blocks but latency will be lower.
    /// * `read_callback` - Required callback that is called to allow you to process audio data from the instream. See `InStreamReader` for more details.
    /// * `overflow_callback` - Optional callback that is called when your `read_callback` is too slow and skips some input.
    /// * `error_callback` - Optional error callback.
    ///
    /// Currently it is not possible to set the outstream name, or libsoundio's `non_terminal_hint`.
    ///
    /// # Return Values
    ///
    /// If successful the function returns an `InStream` which you can call `InStream::start()` on,
    /// otherwise it returns one of the following errors:
    ///
    /// * `Error::Invalid`
    ///   - `aim()` is not `DeviceAim::Input`
    ///   - `format` is not valid
    ///   - `channel_count` is greater than `SOUNDIO_MAX_CHANNELS` (24).
    /// * `Error::NoMem`
    /// * `Error::OpeningDevice`
    /// * `Error::BackendDisconnected`
    /// * `Error::SystemResources`
    /// * `Error::NoSuchClient` - when JACK returns `JackNoSuchClient`
    /// * `Error::IncompatibleBackend` - `OutStream::channel_count()` is greater than the number of channels the backend can handle.
    /// * `Error::IncompatibleDevice` - stream parameters requested are not compatible with the chosen device.
    ///
    /// # Lifetimes
    ///
    /// `'a` is the lifetime of the `Device`. The `InStream` lifetime `'b` must be less than or equal to `'a` (indicated by `'b: 'a`).
    /// Also the callbacks must have a lifetime greater than or equal to `'b`. They do not need to be `'static`.
    pub fn open_instream<'b: 'a, ReadCB, OverflowCB, ErrorCB>(
        &'a self,
        sample_rate: i32,
        format: Format,
        layout: ChannelLayout,
        latency: f64,
        read_callback: ReadCB,
        overflow_callback: Option<OverflowCB>,
        error_callback: Option<ErrorCB>,
    ) -> Result<InStream<'b>>
    where
        ReadCB: 'b + FnMut(&mut InStreamReader),
        OverflowCB: 'b + FnMut(),
        ErrorCB: 'b + FnMut(Error),
    {
        let mut instream = unsafe { raw::soundio_instream_create(self.device) };
        if instream.is_null() {
            // Note that we should really abort() here (that's what the rest of Rust
            // does on OOM), but there is no stable way to abort in Rust that I can see.
            panic!("soundio_instream_create() failed (out of memory).");
        }

        unsafe {
            (*instream).sample_rate = sample_rate;
            (*instream).format = format.into();
            (*instream).layout = layout.into();
            (*instream).software_latency = latency;
            (*instream).read_callback = instream_read_callback;
            (*instream).overflow_callback = Some(instream_overflow_callback);
            (*instream).error_callback = Some(instream_error_callback);
        }

        let mut stream = InStream {
            userdata: Box::new(InStreamUserData {
                instream,
                read_callback: Box::new(read_callback),
                overflow_callback: match overflow_callback {
                    Some(cb) => Some(Box::new(cb)),
                    None => None,
                },
                error_callback: match error_callback {
                    Some(cb) => Some(Box::new(cb)),
                    None => None,
                },
            }),
            phantom: PhantomData,
        };

        // Safe userdata pointer.
        unsafe {
            (*stream.userdata.instream).userdata =
                stream.userdata.as_mut() as *mut InStreamUserData as *mut _;
        }

        match unsafe { raw::soundio_instream_open(stream.userdata.instream) } {
            0 => {}
            x => return Err(x.into()),
        };

        match unsafe { (*stream.userdata.instream).layout_error } {
            0 => {}
            x => return Err(x.into()),
        }

        Ok(stream)
    }
}

impl<'a> Drop for Device<'a> {
    fn drop(&mut self) {
        unsafe {
            raw::soundio_device_unref(self.device);
        }
    }
}

impl<'a> Display for Device<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
