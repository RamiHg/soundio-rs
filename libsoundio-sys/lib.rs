#![allow(dead_code)]

extern crate libc;

use std::os::raw::{c_char, c_void, c_int, c_double, c_float};

// There is no c_bool, but you can use Rust's i8 or bool instead.
// See https://github.com/rust-lang/rfcs/pull/954#issuecomment-169820630
#[allow(non_camel_case_types)]
type c_bool = i8;

/** \mainpage
 *
 * \section intro_sec Overview
 *
 * libsoundio is a C library for cross-platform audio input and output. It is
 * suitable for real-time and consumer software.
 *
 * Documentation: soundio.h
 */


/** \example sio_list_devices.c
 * List the available input and output devices on the system and their
 * properties. Supports watching for changes and specifying backend to use.
 */

/** \example sio_sine.c
 * Play a sine wave over the default output device.
 * Supports specifying device and backend to use.
 */

/** \example sio_record.c
 * Record audio to an output file.
 * Supports specifying device and backend to use.
 */

/** \example sio_microphone.c
 * Stream the default input device over the default output device.
 * Supports specifying device and backend to use.
 */

/** \example backend_disconnect_recover.c
 * Demonstrates recovering from a backend disconnecting.
 */

// See also ::soundio_strerror
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SoundIoError {
    SoundIoErrorNone,
    // Out of memory.
    SoundIoErrorNoMem,
    // The backend does not appear to be active or running.
    SoundIoErrorInitAudioBackend,
    // A system resource other than memory was not available.
    SoundIoErrorSystemResources,
    // Attempted to open a device and failed.
    SoundIoErrorOpeningDevice,
    SoundIoErrorNoSuchDevice,
    // The programmer did not comply with the API.
    SoundIoErrorInvalid,
    // libsoundio was compiled without support for that backend.
    SoundIoErrorBackendUnavailable,
    // An open stream had an error that can only be recovered from by
    // destroying the stream and creating it again.
    SoundIoErrorStreaming,
    // Attempted to use a device with parameters it cannot support.
    SoundIoErrorIncompatibleDevice,
    // When JACK returns `JackNoSuchClient`
    SoundIoErrorNoSuchClient,
    // Attempted to use parameters that the backend cannot support.
    SoundIoErrorIncompatibleBackend,
    // Backend server shutdown or became inactive.
    SoundIoErrorBackendDisconnected,
    SoundIoErrorInterrupted,
    // Buffer underrun occurred.
    SoundIoErrorUnderflow,
    // Unable to convert to or from UTF-8 to the native string format.
    SoundIoErrorEncodingString,
}

// Specifies where a channel is physically located.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SoundIoChannelId {
    SoundIoChannelIdInvalid,

    SoundIoChannelIdFrontLeft, // First of the more commonly supported ids.
    SoundIoChannelIdFrontRight,
    SoundIoChannelIdFrontCenter,
    SoundIoChannelIdLfe,
    SoundIoChannelIdBackLeft,
    SoundIoChannelIdBackRight,
    SoundIoChannelIdFrontLeftCenter,
    SoundIoChannelIdFrontRightCenter,
    SoundIoChannelIdBackCenter,
    SoundIoChannelIdSideLeft,
    SoundIoChannelIdSideRight,
    SoundIoChannelIdTopCenter,
    SoundIoChannelIdTopFrontLeft,
    SoundIoChannelIdTopFrontCenter,
    SoundIoChannelIdTopFrontRight,
    SoundIoChannelIdTopBackLeft,
    SoundIoChannelIdTopBackCenter,
    SoundIoChannelIdTopBackRight, // Last of the more commonly supported ids.

    SoundIoChannelIdBackLeftCenter, // First of the less commonly supported ids.
    SoundIoChannelIdBackRightCenter,
    SoundIoChannelIdFrontLeftWide,
    SoundIoChannelIdFrontRightWide,
    SoundIoChannelIdFrontLeftHigh,
    SoundIoChannelIdFrontCenterHigh,
    SoundIoChannelIdFrontRightHigh,
    SoundIoChannelIdTopFrontLeftCenter,
    SoundIoChannelIdTopFrontRightCenter,
    SoundIoChannelIdTopSideLeft,
    SoundIoChannelIdTopSideRight,
    SoundIoChannelIdLeftLfe,
    SoundIoChannelIdRightLfe,
    SoundIoChannelIdLfe2,
    SoundIoChannelIdBottomCenter,
    SoundIoChannelIdBottomLeftCenter,
    SoundIoChannelIdBottomRightCenter,

    // Mid/side recording
    SoundIoChannelIdMsMid,
    SoundIoChannelIdMsSide,

    // first order ambisonic channels
    SoundIoChannelIdAmbisonicW,
    SoundIoChannelIdAmbisonicX,
    SoundIoChannelIdAmbisonicY,
    SoundIoChannelIdAmbisonicZ,

    // X-Y Recording
    SoundIoChannelIdXyX,
    SoundIoChannelIdXyY,

    SoundIoChannelIdHeadphonesLeft, // First of the "other" channel ids
    SoundIoChannelIdHeadphonesRight,
    SoundIoChannelIdClickTrack,
    SoundIoChannelIdForeignLanguage,
    SoundIoChannelIdHearingImpaired,
    SoundIoChannelIdNarration,
    SoundIoChannelIdHaptic,
    SoundIoChannelIdDialogCentricMix, // Last of the "other" channel ids

    SoundIoChannelIdAux,
    SoundIoChannelIdAux0,
    SoundIoChannelIdAux1,
    SoundIoChannelIdAux2,
    SoundIoChannelIdAux3,
    SoundIoChannelIdAux4,
    SoundIoChannelIdAux5,
    SoundIoChannelIdAux6,
    SoundIoChannelIdAux7,
    SoundIoChannelIdAux8,
    SoundIoChannelIdAux9,
    SoundIoChannelIdAux10,
    SoundIoChannelIdAux11,
    SoundIoChannelIdAux12,
    SoundIoChannelIdAux13,
    SoundIoChannelIdAux14,
    SoundIoChannelIdAux15,
}

// Built-in channel layouts for convenience.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SoundIoChannelLayoutId {
    SoundIoChannelLayoutIdMono,
    SoundIoChannelLayoutIdStereo,
    SoundIoChannelLayoutId2Point1,
    SoundIoChannelLayoutId3Point0,
    SoundIoChannelLayoutId3Point0Back,
    SoundIoChannelLayoutId3Point1,
    SoundIoChannelLayoutId4Point0,
    SoundIoChannelLayoutIdQuad,
    SoundIoChannelLayoutIdQuadSide,
    SoundIoChannelLayoutId4Point1,
    SoundIoChannelLayoutId5Point0Back,
    SoundIoChannelLayoutId5Point0Side,
    SoundIoChannelLayoutId5Point1,
    SoundIoChannelLayoutId5Point1Back,
    SoundIoChannelLayoutId6Point0Side,
    SoundIoChannelLayoutId6Point0Front,
    SoundIoChannelLayoutIdHexagonal,
    SoundIoChannelLayoutId6Point1,
    SoundIoChannelLayoutId6Point1Back,
    SoundIoChannelLayoutId6Point1Front,
    SoundIoChannelLayoutId7Point0,
    SoundIoChannelLayoutId7Point0Front,
    SoundIoChannelLayoutId7Point1,
    SoundIoChannelLayoutId7Point1Wide,
    SoundIoChannelLayoutId7Point1WideBack,
    SoundIoChannelLayoutIdOctagonal,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SoundIoBackend {
    SoundIoBackendNone,
    SoundIoBackendJack,
    SoundIoBackendPulseAudio,
    SoundIoBackendAlsa,
    SoundIoBackendCoreAudio,
    SoundIoBackendWasapi,
    SoundIoBackendDummy,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SoundIoDeviceAim {
    SoundIoDeviceAimInput,  // capture / recording
    SoundIoDeviceAimOutput, // playback
}

// For your convenience, Native Endian and Foreign Endian constants are defined
// which point to the respective SoundIoFormat values.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SoundIoFormat {
    SoundIoFormatInvalid,
    SoundIoFormatS8,        // Signed 8 bit
    SoundIoFormatU8,        // Unsigned 8 bit
    SoundIoFormatS16LE,     // Signed 16 bit Little Endian
    SoundIoFormatS16BE,     // Signed 16 bit Big Endian
    SoundIoFormatU16LE,     // Unsigned 16 bit Little Endian
    SoundIoFormatU16BE,     // Unsigned 16 bit Little Endian
    SoundIoFormatS24LE,     // Signed 24 bit Little Endian using low three bytes in 32-bit word
    SoundIoFormatS24BE,     // Signed 24 bit Big Endian using low three bytes in 32-bit word
    SoundIoFormatU24LE,     // Unsigned 24 bit Little Endian using low three bytes in 32-bit word
    SoundIoFormatU24BE,     // Unsigned 24 bit Big Endian using low three bytes in 32-bit word
    SoundIoFormatS32LE,     // Signed 32 bit Little Endian
    SoundIoFormatS32BE,     // Signed 32 bit Big Endian
    SoundIoFormatU32LE,     // Unsigned 32 bit Little Endian
    SoundIoFormatU32BE,     // Unsigned 32 bit Big Endian
    SoundIoFormatFloat32LE, // Float 32 bit Little Endian, Range -1.0 to 1.0
    SoundIoFormatFloat32BE, // Float 32 bit Big Endian, Range -1.0 to 1.0
    SoundIoFormatFloat64LE, // Float 64 bit Little Endian, Range -1.0 to 1.0
    SoundIoFormatFloat64BE, // Float 64 bit Big Endian, Range -1.0 to 1.0
}

pub const SOUNDIO_MAX_CHANNELS: usize = 24;

// The size of this struct is OK to use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoChannelLayout {
    pub name: *const c_char, // Note that Copy and Clone are ok because this normally points to an internal static string. I think.
    pub channel_count: c_int,
    pub channels: [SoundIoChannelId; SOUNDIO_MAX_CHANNELS],
}

// The size of this struct is OK to use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoSampleRateRange {
    pub min: c_int,
    pub max: c_int,
}

// The size of this struct is OK to use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundIoChannelArea {
    // Base address of buffer.
    pub ptr: *mut c_char,
    // How many bytes it takes to get from the beginning of one sample to
    // the beginning of the next sample.
    pub step: c_int,
}

// The size of this struct is not part of the API or ABI.
#[repr(C)]
pub struct SoundIo {
    // Optional. Put whatever you want here. Defaults to NULL.
    pub userdata: *mut c_void,
    // Optional callback. Called when the list of devices change. Only called
    // during a call to ::soundio_flush_events or ::soundio_wait_events.
	pub on_devices_change: Option<extern fn(sio: *mut SoundIo)>,
    // Optional callback. Called when the backend disconnects. For example,
    // when the JACK server shuts down. When this happens, listing devices
    // and opening streams will always fail with
    // SoundIoErrorBackendDisconnected. This callback is only called during a
    // call to ::soundio_flush_events or ::soundio_wait_events.
    // If you do not supply a callback, the default will crash your program
    // with an error message. This callback is also called when the thread
    // that retrieves device information runs into an unrecoverable condition
    // such as running out of memory.
    //
    // Possible errors:
    // * #SoundIoErrorBackendDisconnected
    // * #SoundIoErrorNoMem
    // * #SoundIoErrorSystemResources
    // * #SoundIoErrorOpeningDevice - unexpected problem accessing device
    //   information
	pub on_backend_disconnect: Option<extern fn(sio: *mut SoundIo, err: c_int)>,
    // Optional callback. Called from an unknown thread that you should not use
    // to call any soundio functions. You may use this to signal a condition
    // variable to wake up. Called when ::soundio_wait_events would be woken up.
	pub on_events_signal: Option<extern fn(sio: *mut SoundIo)>,

    // Read-only. After calling ::soundio_connect or ::soundio_connect_backend,
    // this field tells which backend is currently connected.
    pub current_backend: SoundIoBackend,

    // Optional: Application name.
    // PulseAudio uses this for "application name".
    // JACK uses this for `client_name`.
    // Must not contain a colon (":").
    pub app_name: *mut c_char,

    // Optional: Real time priority warning.
    // This callback is fired when making thread real-time priority failed. By
    // default, it will print to stderr only the first time it is called
    // a message instructing the user how to configure their system to allow
    // real-time priority threads. This must be set to a function not NULL.
    // To silence the warning, assign this to a function that does nothing.
	pub emit_rtprio_warning: Option<extern fn()>,

    // Optional: JACK info callback.
    // By default, libsoundio sets this to an empty function in order to
    // silence stdio messages from JACK. You may override the behavior by
    // setting this to `NULL` or providing your own function. This is
    // registered with JACK regardless of whether ::soundio_connect_backend
    // succeeds.
	pub jack_info_callback: Option<extern fn(msg: *const c_char)>,
    // Optional: JACK error callback.
    // See SoundIo::jack_info_callback
	pub jack_error_callback: Option<extern fn(msg: *const c_char)>,
}

// The size of this struct is not part of the API or ABI.
#[repr(C)]
pub struct SoundIoDevice {
    // Read-only. Set automatically.
    pub soundio: *mut SoundIo,

    // A string of bytes that uniquely identifies this device.
    // If the same physical device supports both input and output, that makes
    // one SoundIoDevice for the input and one SoundIoDevice for the output.
    // In this case, the id of each SoundIoDevice will be the same, and
    // SoundIoDevice::aim will be different. Additionally, if the device
    // supports raw mode, there may be up to four devices with the same id:
    // one for each value of SoundIoDevice::is_raw and one for each value of
    // SoundIoDevice::aim.
    pub id: *mut c_char,
    // User-friendly UTF-8 encoded text to describe the device.
    pub name: *mut c_char,

    // Tells whether this device is an input device or an output device.
    pub aim: SoundIoDeviceAim,

    // Channel layouts are handled similarly to SoundIoDevice::formats.
    // If this information is missing due to a SoundIoDevice::probe_error,
    // layouts will be NULL. It's OK to modify this data, for example calling
    // ::soundio_sort_channel_layouts on it.
    // Devices are guaranteed to have at least 1 channel layout.
    pub layouts: *mut SoundIoChannelLayout,
    pub layout_count: c_int,
    // See SoundIoDevice::current_format
    pub current_layout: SoundIoChannelLayout,

    // List of formats this device supports. See also
    // SoundIoDevice::current_format.
    pub formats: *mut SoundIoFormat,
    // How many formats are available in SoundIoDevice::formats.
    pub format_count: c_int,
    // A device is either a raw device or it is a virtual device that is
    // provided by a software mixing service such as dmix or PulseAudio (see
    // SoundIoDevice::is_raw). If it is a raw device,
    // current_format is meaningless;
    // the device has no current format until you open it. On the other hand,
    // if it is a virtual device, current_format describes the
    // destination sample format that your audio will be converted to. Or,
    // if you're the lucky first application to open the device, you might
    // cause the current_format to change to your format.
    // Generally, you want to ignore current_format and use
    // whatever format is most convenient
    // for you which is supported by the device, because when you are the only
    // application left, the mixer might decide to switch
    // current_format to yours. You can learn the supported formats via
    // formats and SoundIoDevice::format_count. If this information is missing
    // due to a probe error, formats will be `NULL`. If current_format is
    // unavailable, it will be set to #SoundIoFormatInvalid.
    // Devices are guaranteed to have at least 1 format available.
    pub current_format: SoundIoFormat,

    // Sample rate is the number of frames per second.
    // Sample rate is handled very similar to SoundIoDevice::formats.
    // If sample rate information is missing due to a probe error, the field
    // will be set to NULL.
    // Devices which have SoundIoDevice::probe_error set to #SoundIoErrorNone are
    // guaranteed to have at least 1 sample rate available.
    pub sample_rates: *mut SoundIoSampleRateRange,
    // How many sample rate ranges are available in
    // SoundIoDevice::sample_rates. 0 if sample rate information is missing
    // due to a probe error.
    pub sample_rate_count: c_int,
    // See SoundIoDevice::current_format
    // 0 if sample rate information is missing due to a probe error.
    pub sample_rate_current: c_int,

    // Software latency minimum in seconds. If this value is unknown or
    // irrelevant, it is set to 0.0.
    // For PulseAudio and WASAPI this value is unknown until you open a
    // stream.
    pub software_latency_min: c_double,
    // Software latency maximum in seconds. If this value is unknown or
    // irrelevant, it is set to 0.0.
    // For PulseAudio and WASAPI this value is unknown until you open a
    // stream.
    pub software_latency_max: c_double,
    // Software latency in seconds. If this value is unknown or
    // irrelevant, it is set to 0.0.
    // For PulseAudio and WASAPI this value is unknown until you open a
    // stream.
    // See SoundIoDevice::current_format
    pub software_latency_current: c_double,

    // Raw means that you are directly opening the hardware device and not
    // going through a proxy such as dmix, PulseAudio, or JACK. When you open a
    // raw device, other applications on the computer are not able to
    // simultaneously access the device. Raw devices do not perform automatic
    // resampling and thus tend to have fewer formats available.
    pub is_raw: c_bool,

    // Devices are reference counted. See ::soundio_device_ref and
    // ::soundio_device_unref.
    pub ref_count: c_int,

    // This is set to a SoundIoError representing the result of the device
    // probe. Ideally this will be SoundIoErrorNone in which case all the
    // fields of the device will be populated. If there is an error code here
    // then information about formats, sample rates, and channel layouts might
    // be missing.
    //
    // Possible errors:
    // * #SoundIoErrorOpeningDevice
    // * #SoundIoErrorNoMem
    pub probe_error: c_int,
}

// The size of this struct is not part of the API or ABI.
#[repr(C)]
pub struct SoundIoOutStream {
    // Populated automatically when you call ::soundio_outstream_create.
    pub device: *mut SoundIoDevice,

    // Defaults to #SoundIoFormatFloat32NE, followed by the first one
    // supported.
    pub format: SoundIoFormat,

    // Sample rate is the number of frames per second.
    // Defaults to 48000 (and then clamped into range).
    pub sample_rate: c_int,

    // Defaults to Stereo, if available, followed by the first layout
    // supported.
    pub layout: SoundIoChannelLayout,

    // Ignoring hardware latency, this is the number of seconds it takes for
    // the last sample in a full buffer to be played.
    // After you call ::soundio_outstream_open, this value is replaced with the
    // actual software latency, as near to this value as possible.
    // On systems that support clearing the buffer, this defaults to a large
    // latency, potentially upwards of 2 seconds, with the understanding that
    // you will call ::soundio_outstream_clear_buffer when you want to reduce
    // the latency to 0. On systems that do not support clearing the buffer,
    // this defaults to a reasonable lower latency value.
    //
    // On backends with high latencies (such as 2 seconds), `frame_count_min`
    // will be 0, meaning you don't have to fill the entire buffer. In this
    // case, the large buffer is there if you want it; you only have to fill
    // as much as you want. On backends like JACK, `frame_count_min` will be
    // equal to `frame_count_max` and if you don't fill that many frames, you
    // will get glitches.
    //
    // If the device has unknown software latency min and max values, you may
    // still set this, but you might not get the value you requested.
    // For PulseAudio, if you set this value to non-default, it sets
    // `PA_STREAM_ADJUST_LATENCY` and is the value used for `maxlength` and
    // `tlength`.
    //
    // For JACK, this value is always equal to
    // SoundIoDevice::software_latency_current of the device.
    pub software_latency: c_double,
    pub volume: c_float,

    // Defaults to NULL. Put whatever you want here.
    pub userdata: *mut c_void,
    // In this callback, you call ::soundio_outstream_begin_write and
    // ::soundio_outstream_end_write as many times as necessary to write
    // at minimum `frame_count_min` frames and at maximum `frame_count_max`
    // frames. `frame_count_max` will always be greater than 0. Note that you
    // should write as many frames as you can; `frame_count_min` might be 0 and
    // you can still get a buffer underflow if you always write
    // `frame_count_min` frames.
    //
    // For Dummy, ALSA, and PulseAudio, `frame_count_min` will be 0. For JACK
    // and CoreAudio `frame_count_min` will be equal to `frame_count_max`.
    //
    // The code in the supplied function must be suitable for real-time
    // execution. That means that it cannot call functions that might block
    // for a long time. This includes all I/O functions (disk, TTY, network),
    // malloc, free, printf, pthread_mutex_lock, sleep, wait, poll, select,
    // pthread_join, pthread_cond_wait, etc.
	pub write_callback: extern fn(stream: *mut SoundIoOutStream, frame_count_min: c_int, frame_count_max: c_int),
    // This optional callback happens when the sound device runs out of
    // buffered audio data to play. After this occurs, the outstream waits
    // until the buffer is full to resume playback.
    // This is called from the SoundIoOutStream::write_callback thread context.
	pub underflow_callback: Option<extern fn(stream: *mut SoundIoOutStream)>,
    // Optional callback. `err` is always SoundIoErrorStreaming.
    // SoundIoErrorStreaming is an unrecoverable error. The stream is in an
    // invalid state and must be destroyed.
    // If you do not supply error_callback, the default callback will print
    // a message to stderr and then call `abort`.
    // This is called from the SoundIoOutStream::write_callback thread context.
	pub error_callback: Option<extern fn(stream: *mut SoundIoOutStream, err: c_int)>,

    // Optional: Name of the stream. Defaults to "SoundIoOutStream"
    // PulseAudio uses this for the stream name.
    // JACK uses this for the client name of the client that connects when you
    // open the stream.
    // WASAPI uses this for the session display name.
    // Must not contain a colon (":").
    pub name: *const c_char,

    // Optional: Hint that this output stream is nonterminal. This is used by
    // JACK and it means that the output stream data originates from an input
    // stream. Defaults to `false`.
    pub non_terminal_hint: c_bool,


    // computed automatically when you call ::soundio_outstream_open
    pub bytes_per_frame: c_int,
    // computed automatically when you call ::soundio_outstream_open
    pub bytes_per_sample: c_int,

    // If setting the channel layout fails for some reason, this field is set
    // to an error code. Possible error codes are:
    // * #SoundIoErrorIncompatibleDevice
    pub layout_error: c_int,
}

// The size of this struct is not part of the API or ABI.
#[repr(C)]
pub struct SoundIoInStream {
    // Populated automatically when you call ::soundio_outstream_create.
    pub device: *mut SoundIoDevice,

    // Defaults to #SoundIoFormatFloat32NE, followed by the first one
    // supported.
    pub format: SoundIoFormat,

    // Sample rate is the number of frames per second.
    // Defaults to max(sample_rate_min, min(sample_rate_max, 48000))
    pub sample_rate: c_int,

    // Defaults to Stereo, if available, followed by the first layout
    // supported.
    pub layout: SoundIoChannelLayout,

    // Ignoring hardware latency, this is the number of seconds it takes for a
    // captured sample to become available for reading.
    // After you call ::soundio_instream_open, this value is replaced with the
    // actual software latency, as near to this value as possible.
    // A higher value means less CPU usage. Defaults to a large value,
    // potentially upwards of 2 seconds.
    // If the device has unknown software latency min and max values, you may
    // still set this, but you might not get the value you requested.
    // For PulseAudio, if you set this value to non-default, it sets
    // `PA_STREAM_ADJUST_LATENCY` and is the value used for `fragsize`.
    // For JACK, this value is always equal to
    // SoundIoDevice::software_latency_current
    pub software_latency: c_double,

    // Defaults to NULL. Put whatever you want here.
    pub userdata: *mut c_void,
    // In this function call ::soundio_instream_begin_read and
    // ::soundio_instream_end_read as many times as necessary to read at
    // minimum `frame_count_min` frames and at maximum `frame_count_max`
    // frames. If you return from read_callback without having read
    // `frame_count_min`, the frames will be dropped. `frame_count_max` is how
    // many frames are available to read.
    //
    // The code in the supplied function must be suitable for real-time
    // execution. That means that it cannot call functions that might block
    // for a long time. This includes all I/O functions (disk, TTY, network),
    // malloc, free, printf, pthread_mutex_lock, sleep, wait, poll, select,
    // pthread_join, pthread_cond_wait, etc.
	pub read_callback: extern fn(stream: *mut SoundIoInStream, frame_count_min: c_int, frame_count_max: c_int),
    // This optional callback happens when the sound device buffer is full,
    // yet there is more captured audio to put in it.
    // This is never fired for PulseAudio.
    // This is called from the SoundIoInStream::read_callback thread context.
	pub overflow_callback: Option<extern fn(stream: *mut SoundIoInStream)>,
    // Optional callback. `err` is always SoundIoErrorStreaming.
    // SoundIoErrorStreaming is an unrecoverable error. The stream is in an
    // invalid state and must be destroyed.
    // If you do not supply `error_callback`, the default callback will print
    // a message to stderr and then abort().
    // This is called from the SoundIoInStream::read_callback thread context.
	pub error_callback: Option<extern fn(stream: *mut SoundIoInStream, err: c_int)>,

    // Optional: Name of the stream. Defaults to "SoundIoInStream";
    // PulseAudio uses this for the stream name.
    // JACK uses this for the client name of the client that connects when you
    // open the stream.
    // WASAPI uses this for the session display name.
    // Must not contain a colon (":").
    pub name: *const c_char,

    // Optional: Hint that this input stream is nonterminal. This is used by
    // JACK and it means that the data received by the stream will be
    // passed on or made available to another stream. Defaults to `false`.
    pub non_terminal_hint: c_bool,

    // computed automatically when you call ::soundio_instream_open
    pub bytes_per_frame: c_int,
    // computed automatically when you call ::soundio_instream_open
    pub bytes_per_sample: c_int,

    // If setting the channel layout fails for some reason, this field is set
    // to an error code. Possible error codes are: #SoundIoErrorIncompatibleDevice
    pub layout_error: c_int,
}

pub enum SoundIoRingBuffer {
}

extern {
		
	// See also ::soundio_version_major, ::soundio_version_minor, ::soundio_version_patch
	pub fn soundio_version_string() -> *const c_char;
	// See also ::soundio_version_string, ::soundio_version_minor, ::soundio_version_patch
	pub fn soundio_version_major() -> c_int;
	// See also ::soundio_version_major, ::soundio_version_string, ::soundio_version_patch
	pub fn soundio_version_minor() -> c_int;
	// See also ::soundio_version_major, ::soundio_version_minor, ::soundio_version_string
	pub fn soundio_version_patch() -> c_int;

	// Create a SoundIo context. You may create multiple instances of this to
	// connect to multiple backends. Sets all fields to defaults.
	// Returns `NULL` if and only if memory could not be allocated.
	// See also ::soundio_destroy
	pub fn soundio_create() -> *mut SoundIo;
	pub fn soundio_destroy(soundio: *mut SoundIo);


	// Tries ::soundio_connect_backend on all available backends in order.
	// Possible errors:
	// * #SoundIoErrorInvalid - already connected
	// * #SoundIoErrorNoMem
	// * #SoundIoErrorSystemResources
	// * #SoundIoErrorNoSuchClient - when JACK returns `JackNoSuchClient`
	// See also ::soundio_disconnect
	pub fn soundio_connect(soundio: *mut SoundIo) -> c_int;
	// Instead of calling ::soundio_connect you may call this function to try a
	// specific backend.
	// Possible errors:
	// * #SoundIoErrorInvalid - already connected or invalid backend parameter
	// * #SoundIoErrorNoMem
	// * #SoundIoErrorBackendUnavailable - backend was not compiled in
	// * #SoundIoErrorSystemResources
	// * #SoundIoErrorNoSuchClient - when JACK returns `JackNoSuchClient`
	// * #SoundIoErrorInitAudioBackend - requested `backend` is not active
	// * #SoundIoErrorBackendDisconnected - backend disconnected while connecting
	// See also ::soundio_disconnect
	pub fn soundio_connect_backend(soundio: *mut SoundIo, backend: SoundIoBackend) -> c_int;
	pub fn soundio_disconnect(soundio: *mut SoundIo);

	// Get a string representation of a #SoundIoError
	pub fn soundio_strerror(error: c_int) -> *const c_char;
	// Get a string representation of a #SoundIoBackend
	pub fn soundio_backend_name(backend: SoundIoBackend) -> *const c_char;

	// Returns the number of available backends.
	pub fn soundio_backend_count(soundio: *mut SoundIo) -> c_int;
	// get the available backend at the specified index
	// (0 <= index < ::soundio_backend_count)
	pub fn soundio_get_backend(soundio: *mut SoundIo, index: c_int) -> SoundIoBackend;

	// Returns whether libsoundio was compiled with backend.
	pub fn soundio_have_backend(backend: SoundIoBackend) -> c_bool;

	// Atomically update information for all connected devices. Note that calling
	// this function merely flips a pointer; the actual work of collecting device
	// information is done elsewhere. It is performant to call this function many
	// times per second.
	//
	// When you call this, the following callbacks might be called:
	// * SoundIo::on_devices_change
	// * SoundIo::on_backend_disconnect
	// This is the only time those callbacks can be called.
	//
	// This must be called from the same thread as the thread in which you call
	// these functions:
	// * ::soundio_input_device_count
	// * ::soundio_output_device_count
	// * ::soundio_get_input_device
	// * ::soundio_get_output_device
	// * ::soundio_default_input_device_index
	// * ::soundio_default_output_device_index
	//
	// Note that if you do not care about learning about updated devices, you
	// might call this function only once ever and never call
	// ::soundio_wait_events.
	pub fn soundio_flush_events(soundio: *mut SoundIo);

	// This function calls ::soundio_flush_events then blocks until another event
	// is ready or you call ::soundio_wakeup. Be ready for spurious wakeups.
	pub fn soundio_wait_events(soundio: *mut SoundIo);

	// Makes ::soundio_wait_events stop blocking.
	pub fn soundio_wakeup(soundio: *mut SoundIo);


	// If necessary you can manually trigger a device rescan. Normally you will
	// not ever have to call this function, as libsoundio listens to system events
	// for device changes and responds to them by rescanning devices and preparing
	// the new device information for you to be atomically replaced when you call
	// ::soundio_flush_events. However you might run into cases where you want to
	// force trigger a device rescan, for example if an ALSA device has a
	// SoundIoDevice::probe_error.
	//
	// After you call this you still have to use ::soundio_flush_events or
	// ::soundio_wait_events and then wait for the
	// SoundIo::on_devices_change callback.
	//
	// This can be called from any thread context except for
	// SoundIoOutStream::write_callback and SoundIoInStream::read_callback
	pub fn soundio_force_device_scan(soundio: *mut SoundIo);


	// Channel Layouts

	// Returns whether the channel count field and each channel id matches in
	// the supplied channel layouts.
	pub fn soundio_channel_layout_equal(
			a: *mut SoundIoChannelLayout,
			b: *mut SoundIoChannelLayout) -> c_bool;

	pub fn soundio_get_channel_name(id: SoundIoChannelId) -> *const c_char;
	// Given UTF-8 encoded text which is the name of a channel such as
	// "Front Left", "FL", or "front-left", return the corresponding
	// SoundIoChannelId. Returns SoundIoChannelIdInvalid for no match.
	pub fn soundio_parse_channel_id(str: *const c_char, str_len: c_int) -> SoundIoChannelId;

	// Returns the number of builtin channel layouts.
	pub fn soundio_channel_layout_builtin_count() -> c_int;
	// Returns a builtin channel layout. 0 <= `index` < ::soundio_channel_layout_builtin_count
	//
	// Although `index` is of type `int`, it should be a valid
	// #SoundIoChannelLayoutId enum value.
	pub fn soundio_channel_layout_get_builtin(index: c_int) -> *const SoundIoChannelLayout;

	// Get the default builtin channel layout for the given number of channels.
	pub fn soundio_channel_layout_get_default(channel_count: c_int) -> *const SoundIoChannelLayout;

	// Return the index of `channel` in `layout`, or `-1` if not found.
	pub fn soundio_channel_layout_find_channel(
			layout: *const SoundIoChannelLayout, channel: SoundIoChannelId) -> c_int;

	// Populates the name field of layout if it matches a builtin one.
	// returns whether it found a match
	pub fn soundio_channel_layout_detect_builtin(layout: *mut SoundIoChannelLayout) -> c_bool;

	// Iterates over preferred_layouts. Returns the first channel layout in
	// preferred_layouts which matches one of the channel layouts in
	// available_layouts. Returns NULL if none matches.
	pub fn soundio_best_matching_channel_layout(
			preferred_layouts: *const SoundIoChannelLayout, preferred_layout_count: c_int,
			available_layouts: *const SoundIoChannelLayout, available_layout_count: c_int) -> *const SoundIoChannelLayout;

	// Sorts by channel count, descending.
	pub fn soundio_sort_channel_layouts(layout: *mut SoundIoChannelLayout, layout_count: c_int);

	// Sample Formats

	// Returns -1 on invalid format.
	pub fn soundio_get_bytes_per_sample(format: SoundIoFormat) -> c_int;

	// A frame is one sample per channel.
	// static inline int soundio_get_bytes_per_frame(enum SoundIoFormat format, channel_count: c_int) {
	// 	return soundio_get_bytes_per_sample(format) * channel_count;
	// }

	// Sample rate is the number of frames per second.
	// static inline int soundio_get_bytes_per_second(enum SoundIoFormat format,
	// 		channel_count: c_int, int sample_rate)
	// {
	// 	return soundio_get_bytes_per_frame(format, channel_count) * sample_rate;
	// }

	// Returns string representation of `format`.
	pub fn soundio_format_string(format: SoundIoFormat) -> *const c_char;




	// Devices

	// When you call ::soundio_flush_events, a snapshot of all device state is
	// saved and these functions merely access the snapshot data. When you want
	// to check for new devices, call ::soundio_flush_events. Or you can call
	// ::soundio_wait_events to block until devices change. If an error occurs
	// scanning devices in a background thread, SoundIo::on_backend_disconnect is called
	// with the error code.

	// Get the number of input devices.
	// Returns -1 if you never called ::soundio_flush_events.
	pub fn soundio_input_device_count(soundio: *mut SoundIo) -> c_int;
	// Get the number of output devices.
	// Returns -1 if you never called ::soundio_flush_events.
	pub fn soundio_output_device_count(soundio: *mut SoundIo) -> c_int;

	// Always returns a device. Call ::soundio_device_unref when done.
	// `index` must be 0 <= index < ::soundio_input_device_count
	// Returns NULL if you never called ::soundio_flush_events or if you provide
	// invalid parameter values.
	pub fn soundio_get_input_device(soundio: *mut SoundIo, index: c_int) -> *mut SoundIoDevice;
	// Always returns a device. Call ::soundio_device_unref when done.
	// `index` must be 0 <= index < ::soundio_output_device_count
	// Returns NULL if you never called ::soundio_flush_events or if you provide
	// invalid parameter values.
	pub fn soundio_get_output_device(soundio: *mut SoundIo, index: c_int) -> *mut SoundIoDevice;

	// returns the index of the default input device
	// returns -1 if there are no devices or if you never called
	// ::soundio_flush_events.
	pub fn soundio_default_input_device_index(soundio: *mut SoundIo) -> c_int;

	// returns the index of the default output device
	// returns -1 if there are no devices or if you never called
	// ::soundio_flush_events.
	pub fn soundio_default_output_device_index(soundio: *mut SoundIo) -> c_int;

	// Add 1 to the reference count of `device`.
	pub fn soundio_device_ref(device: *mut SoundIoDevice);
	// Remove 1 to the reference count of `device`. Clean up if it was the last
	// reference.
	pub fn soundio_device_unref(device: *mut SoundIoDevice);

	// Return `true` if and only if the devices have the same SoundIoDevice::id,
	// SoundIoDevice::is_raw, and SoundIoDevice::aim are the same.
	pub fn soundio_device_equal(
			a: *const SoundIoDevice,
			b: *const SoundIoDevice) -> c_bool;

	// Sorts channel layouts by channel count, descending.
	pub fn soundio_device_sort_channel_layouts(device: *mut SoundIoDevice);

	// Convenience function. Returns whether `format` is included in the device's
	// supported formats.
	pub fn soundio_device_supports_format(device: *mut SoundIoDevice,
			format: SoundIoFormat) -> c_bool;

	// Convenience function. Returns whether `layout` is included in the device's
	// supported channel layouts.
	pub fn soundio_device_supports_layout(device: *mut SoundIoDevice,
			layout: *const SoundIoChannelLayout) -> c_bool;

	// Convenience function. Returns whether `sample_rate` is included in the
	// device's supported sample rates.
	pub fn soundio_device_supports_sample_rate(device: *mut SoundIoDevice,
			sample_rate: c_int) -> c_bool;

	// Convenience function. Returns the available sample rate nearest to
	// `sample_rate`, rounding up.
	pub fn soundio_device_nearest_sample_rate(device: *mut SoundIoDevice,
			sample_rate: c_int) -> c_int;



	// Output Streams
	// Allocates memory and sets defaults. Next you should fill out the struct fields
	// and then call ::soundio_outstream_open. Sets all fields to defaults.
	// Returns `NULL` if and only if memory could not be allocated.
	// See also ::soundio_outstream_destroy
	pub fn soundio_outstream_create(device: *mut SoundIoDevice) -> *mut SoundIoOutStream;
	// You may not call this function from the SoundIoOutStream::write_callback thread context.
	pub fn soundio_outstream_destroy(outstream: *mut SoundIoOutStream);

	// After you call this function, SoundIoOutStream::software_latency is set to
	// the correct value.
	//
	// The next thing to do is call ::soundio_instream_start.
	// If this function returns an error, the outstream is in an invalid state and
	// you must call ::soundio_outstream_destroy on it.
	//
	// Possible errors:
	// * #SoundIoErrorInvalid
	//   * SoundIoDevice::aim is not #SoundIoDeviceAimOutput
	//   * SoundIoOutStream::format is not valid
	//   * SoundIoOutStream::channel_count is greater than #SOUNDIO_MAX_CHANNELS
	// * #SoundIoErrorNoMem
	// * #SoundIoErrorOpeningDevice
	// * #SoundIoErrorBackendDisconnected
	// * #SoundIoErrorSystemResources
	// * #SoundIoErrorNoSuchClient - when JACK returns `JackNoSuchClient`
	// * #SoundIoErrorOpeningDevice
	// * #SoundIoErrorIncompatibleBackend - SoundIoOutStream::channel_count is
	//   greater than the number of channels the backend can handle.
	// * #SoundIoErrorIncompatibleDevice - stream parameters requested are not
	//   compatible with the chosen device.
	pub fn soundio_outstream_open(outstream: *mut SoundIoOutStream) -> c_int;

	// After you call this function, SoundIoOutStream::write_callback will be called.
	//
	// This function might directly call SoundIoOutStream::write_callback.
	//
	// Possible errors:
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorNoMem
	// * #SoundIoErrorSystemResources
	// * #SoundIoErrorBackendDisconnected
	pub fn soundio_outstream_start(outstream: *mut SoundIoOutStream) -> c_int;

	// Call this function when you are ready to begin writing to the device buffer.
	//  * `outstream` - (in) The output stream you want to write to.
	//  * `areas` - (out) The memory addresses you can write data to, one per
	//    channel. It is OK to modify the pointers if that helps you iterate.
	//  * `frame_count` - (in/out) Provide the number of frames you want to write.
	//    Returned will be the number of frames you can actually write, which is
	//    also the number of frames that will be written when you call
	//    ::soundio_outstream_end_write. The value returned will always be less
	//    than or equal to the value provided.
	// It is your responsibility to call this function exactly as many times as
	// necessary to meet the `frame_count_min` and `frame_count_max` criteria from
	// SoundIoOutStream::write_callback.
	// You must call this function only from the SoundIoOutStream::write_callback thread context.
	// After calling this function, write data to `areas` and then call
	// ::soundio_outstream_end_write.
	// If this function returns an error, do not call ::soundio_outstream_end_write.
	//
	// Possible errors:
	// * #SoundIoErrorInvalid
	//   * `*frame_count` <= 0
	//   * `*frame_count` < `frame_count_min` or `*frame_count` > `frame_count_max`
	//   * function called too many times without respecting `frame_count_max`
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorUnderflow - an underflow caused this call to fail. You might
	//   also get a SoundIoOutStream::underflow_callback, and you might not get
	//   this error code when an underflow occurs. Unlike #SoundIoErrorStreaming,
	//   the outstream is still in a valid state and streaming can continue.
	// * #SoundIoErrorIncompatibleDevice - in rare cases it might just now
	//   be discovered that the device uses non-byte-aligned access, in which
	//   case this error code is returned.
	pub fn soundio_outstream_begin_write(outstream: *mut SoundIoOutStream,
			areas: *mut *mut SoundIoChannelArea, frame_count: *mut c_int) -> c_int;

	// Commits the write that you began with ::soundio_outstream_begin_write.
	// You must call this function only from the SoundIoOutStream::write_callback thread context.
	//
	// Possible errors:
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorUnderflow - an underflow caused this call to fail. You might
	//   also get a SoundIoOutStream::underflow_callback, and you might not get
	//   this error code when an underflow occurs. Unlike #SoundIoErrorStreaming,
	//   the outstream is still in a valid state and streaming can continue.
	pub fn soundio_outstream_end_write(outstream: *mut SoundIoOutStream) -> c_int;

	// Clears the output stream buffer.
	// This function can be called from any thread.
	// This function can be called regardless of whether the outstream is paused
	// or not.
	// Some backends do not support clearing the buffer. On these backends this
	// function will return SoundIoErrorIncompatibleBackend.
	// Some devices do not support clearing the buffer. On these devices this
	// function might return SoundIoErrorIncompatibleDevice.
	// Possible errors:
	//
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorIncompatibleBackend
	// * #SoundIoErrorIncompatibleDevice
	pub fn soundio_outstream_clear_buffer(outstream: *mut SoundIoOutStream) -> c_int;

	// If the underlying backend and device support pausing, this pauses the
	// stream. SoundIoOutStream::write_callback may be called a few more times if
	// the buffer is not full.
	// Pausing might put the hardware into a low power state which is ideal if your
	// software is silent for some time.
	// This function may be called from any thread context, including
	// SoundIoOutStream::write_callback.
	// Pausing when already paused or unpausing when already unpaused has no
	// effect and returns #SoundIoErrorNone.
	//
	// Possible errors:
	// * #SoundIoErrorBackendDisconnected
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorIncompatibleDevice - device does not support
	//   pausing/unpausing. This error code might not be returned even if the
	//   device does not support pausing/unpausing.
	// * #SoundIoErrorIncompatibleBackend - backend does not support
	//   pausing/unpausing.
	// * #SoundIoErrorInvalid - outstream not opened and started
	pub fn soundio_outstream_pause(outstream: *mut SoundIoOutStream, pause: c_bool) -> c_int;

	// Obtain the total number of seconds that the next frame written after the
	// last frame written with ::soundio_outstream_end_write will take to become
	// audible. This includes both software and hardware latency. In other words,
	// if you call this function directly after calling ::soundio_outstream_end_write,
	// this gives you the number of seconds that the next frame written will take
	// to become audible.
	//
	// This function must be called only from within SoundIoOutStream::write_callback.
	//
	// Possible errors:
	// * #SoundIoErrorStreaming
	pub fn soundio_outstream_get_latency(outstream: *mut SoundIoOutStream,
			out_latency: *mut c_double) -> c_int;



	// Input Streams
	// Allocates memory and sets defaults. Next you should fill out the struct fields
	// and then call ::soundio_instream_open. Sets all fields to defaults.
	// Returns `NULL` if and only if memory could not be allocated.
	// See also ::soundio_instream_destroy
	pub fn soundio_instream_create(device: *mut SoundIoDevice) -> *mut SoundIoInStream;
	// You may not call this function from SoundIoInStream::read_callback.
	pub fn soundio_instream_destroy(instream: *mut SoundIoInStream);

	// After you call this function, SoundIoInStream::software_latency is set to the correct
	// value.
	// The next thing to do is call ::soundio_instream_start.
	// If this function returns an error, the instream is in an invalid state and
	// you must call ::soundio_instream_destroy on it.
	//
	// Possible errors:
	// * #SoundIoErrorInvalid
	//   * device aim is not #SoundIoDeviceAimInput
	//   * format is not valid
	//   * requested layout channel count > #SOUNDIO_MAX_CHANNELS
	// * #SoundIoErrorOpeningDevice
	// * #SoundIoErrorNoMem
	// * #SoundIoErrorBackendDisconnected
	// * #SoundIoErrorSystemResources
	// * #SoundIoErrorNoSuchClient
	// * #SoundIoErrorIncompatibleBackend
	// * #SoundIoErrorIncompatibleDevice
	pub fn soundio_instream_open(instream: *mut SoundIoInStream) -> c_int;

	// After you call this function, SoundIoInStream::read_callback will be called.
	//
	// Possible errors:
	// * #SoundIoErrorBackendDisconnected
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorOpeningDevice
	// * #SoundIoErrorSystemResources
	pub fn soundio_instream_start(instream: *mut SoundIoInStream) -> c_int;

	// Call this function when you are ready to begin reading from the device
	// buffer.
	// * `instream` - (in) The input stream you want to read from.
	// * `areas` - (out) The memory addresses you can read data from. It is OK
	//   to modify the pointers if that helps you iterate. There might be a "hole"
	//   in the buffer. To indicate this, `areas` will be `NULL` and `frame_count`
	//   tells how big the hole is in frames.
	// * `frame_count` - (in/out) - Provide the number of frames you want to read;
	//   returns the number of frames you can actually read. The returned value
	//   will always be less than or equal to the provided value. If the provided
	//   value is less than `frame_count_min` from SoundIoInStream::read_callback this function
	//   returns with #SoundIoErrorInvalid.
	// It is your responsibility to call this function no more and no fewer than the
	// correct number of times according to the `frame_count_min` and
	// `frame_count_max` criteria from SoundIoInStream::read_callback.
	// You must call this function only from the SoundIoInStream::read_callback thread context.
	// After calling this function, read data from `areas` and then use
	// ::soundio_instream_end_read` to actually remove the data from the buffer
	// and move the read index forward. ::soundio_instream_end_read should not be
	// called if the buffer is empty (`frame_count` == 0), but it should be called
	// if there is a hole.
	//
	// Possible errors:
	// * #SoundIoErrorInvalid
	//   * `*frame_count` < `frame_count_min` or `*frame_count` > `frame_count_max`
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorIncompatibleDevice - in rare cases it might just now
	//   be discovered that the device uses non-byte-aligned access, in which
	//   case this error code is returned.
	pub fn soundio_instream_begin_read(instream: *mut SoundIoInStream,
			areas: *mut *mut SoundIoChannelArea, frame_count: *mut c_int) -> c_int;
	// This will drop all of the frames from when you called
	// ::soundio_instream_begin_read.
	// You must call this function only from the SoundIoInStream::read_callback thread context.
	// You must call this function only after a successful call to
	// ::soundio_instream_begin_read.
	//
	// Possible errors:
	// * #SoundIoErrorStreaming
	pub fn soundio_instream_end_read(instream: *mut SoundIoInStream) -> c_int;

	// If the underyling device supports pausing, this pauses the stream and
	// prevents SoundIoInStream::read_callback from being called. Otherwise this returns
	// #SoundIoErrorIncompatibleDevice.
	// This function may be called from any thread.
	// Pausing when already paused or unpausing when already unpaused has no
	// effect and always returns #SoundIoErrorNone.
	//
	// Possible errors:
	// * #SoundIoErrorBackendDisconnected
	// * #SoundIoErrorStreaming
	// * #SoundIoErrorIncompatibleDevice - device does not support pausing/unpausing
	pub fn soundio_instream_pause(instream: *mut SoundIoInStream, pause: c_bool) -> c_int;

	// Obtain the number of seconds that the next frame of sound being
	// captured will take to arrive in the buffer, plus the amount of time that is
	// represented in the buffer. This includes both software and hardware latency.
	//
	// This function must be called only from within SoundIoInStream::read_callback.
	//
	// Possible errors:
	// * #SoundIoErrorStreaming
	pub fn soundio_instream_get_latency(instream: *mut SoundIoInStream,
			out_latency: *mut c_double) -> c_int;


	// A ring buffer is a single-reader single-writer lock-free fixed-size queue.
	// libsoundio ring buffers use memory mapping techniques to enable a
	// contiguous buffer when reading or writing across the boundary of the ring
	// buffer's capacity.

	// Enum is defined above. See https://github.com/rust-lang/rust/issues/27303
	// struct SoundIoRingBuffer;

	// `requested_capacity` in bytes.
	// Returns `NULL` if and only if memory could not be allocated.
	// Use ::soundio_ring_buffer_capacity to get the actual capacity, which might
	// be greater for alignment purposes.
	// See also ::soundio_ring_buffer_destroy
	pub fn soundio_ring_buffer_create(soundio: *mut SoundIo, requested_capacity: c_int) -> *mut SoundIoRingBuffer;
	pub fn soundio_ring_buffer_destroy(ring_buffer: *mut SoundIoRingBuffer);

	// When you create a ring buffer, capacity might be more than the requested
	// capacity for alignment purposes. This function returns the actual capacity.
	pub fn soundio_ring_buffer_capacity(ring_buffer: *mut SoundIoRingBuffer) -> c_int;

	// Do not write more than capacity.
	pub fn soundio_ring_buffer_write_ptr(ring_buffer: *mut SoundIoRingBuffer) -> *mut c_char;
	// `count` in bytes.
	pub fn soundio_ring_buffer_advance_write_ptr(ring_buffer: *mut SoundIoRingBuffer, count: c_int);

	// Do not read more than capacity.
	pub fn soundio_ring_buffer_read_ptr(ring_buffer: *mut SoundIoRingBuffer) -> *mut c_char;
	// `count` in bytes.
	pub fn soundio_ring_buffer_advance_read_ptr(ring_buffer: *mut SoundIoRingBuffer, count: c_int);

	// Returns how many bytes of the buffer is used, ready for reading.
	pub fn soundio_ring_buffer_fill_count(ring_buffer: *mut SoundIoRingBuffer) -> c_int;

	// Returns how many bytes of the buffer is free, ready for writing.
	pub fn soundio_ring_buffer_free_count(ring_buffer: *mut SoundIoRingBuffer) -> c_int;

	// Must be called by the writer.
	pub fn soundio_ring_buffer_clear(ring_buffer: *mut SoundIoRingBuffer);

}

