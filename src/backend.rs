extern crate libsoundio_sys as raw;

use super::util::*;

use std::fmt;

/// Backend indicates one of the supported audio backends.
///
/// Linux supports Also, and optionally PulseAudio, and JACK.
///
/// Windows supports Wasapi, and MacOS supports CoreAudio. All platforms
/// support the Dummy backend.
///
/// The Backend type supports the `Display` trait, so you can use it in `println!()`.
///
/// The only use for `Backend::None` is that it is returned from `Context::current_backend()`
/// if the `Context` isn't connected.
///
/// # Examples
///
/// ```
/// println!("The name of PulseAudio is {}", soundio::Backend::PulseAudio);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Backend {
	None,
	Jack,
	PulseAudio,
	Alsa,
	CoreAudio,
	Wasapi,
	Dummy,
}

impl From<raw::SoundIoBackend> for Backend {
    fn from(backend: raw::SoundIoBackend) -> Backend {
		match backend {
			raw::SoundIoBackend::SoundIoBackendJack => Backend::Jack,
			raw::SoundIoBackend::SoundIoBackendPulseAudio => Backend::PulseAudio,
			raw::SoundIoBackend::SoundIoBackendAlsa => Backend::Alsa,
			raw::SoundIoBackend::SoundIoBackendCoreAudio => Backend::CoreAudio,
			raw::SoundIoBackend::SoundIoBackendWasapi => Backend::Wasapi,
			raw::SoundIoBackend::SoundIoBackendDummy => Backend::Dummy,
			_ => Backend::None,
		}
    }
}

impl From<Backend> for raw::SoundIoBackend {
    fn from(backend: Backend) -> raw::SoundIoBackend {
		match backend {
			Backend::Jack => raw::SoundIoBackend::SoundIoBackendJack,
			Backend::PulseAudio => raw::SoundIoBackend::SoundIoBackendPulseAudio,
			Backend::Alsa => raw::SoundIoBackend::SoundIoBackendAlsa,
			Backend::CoreAudio => raw::SoundIoBackend::SoundIoBackendCoreAudio,
			Backend::Wasapi => raw::SoundIoBackend::SoundIoBackendWasapi,
			Backend::Dummy => raw::SoundIoBackend::SoundIoBackendDummy,
			_ => raw::SoundIoBackend::SoundIoBackendNone,
		}
    }
}

impl fmt::Display for Backend {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // In the C source these only use ASCII characters so it is technically ambiguous
        // whether this is UTF-8 or Latin1.
        let s = latin1_to_string( unsafe { raw::soundio_backend_name((*self).into()) } );
		f.write_str(&s)
	}
}