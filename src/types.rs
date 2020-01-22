extern crate libsoundio_sys as raw;

/// Used to identify devices as input or output. In this library
/// all devices are either input or output. If a physical device supports
/// both it is exposed as two devices with the same id, but with different
/// aims returned by `Device::aim()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceAim {
    /// Capture / recording
    Input,
    /// Playback
    Output,
}

impl From<raw::SoundIoDeviceAim> for DeviceAim {
    fn from(aim: raw::SoundIoDeviceAim) -> DeviceAim {
        match aim {
            raw::SoundIoDeviceAim::SoundIoDeviceAimInput => DeviceAim::Input,
            raw::SoundIoDeviceAim::SoundIoDeviceAimOutput => DeviceAim::Output,
        }
    }
}

impl From<DeviceAim> for raw::SoundIoDeviceAim {
    fn from(aim: DeviceAim) -> raw::SoundIoDeviceAim {
        match aim {
            DeviceAim::Input => raw::SoundIoDeviceAim::SoundIoDeviceAimInput,
            DeviceAim::Output => raw::SoundIoDeviceAim::SoundIoDeviceAimOutput,
        }
    }
}

/// Devices report their supported sample rates as ranges. For non-range sample
/// rates `min` and `max` are the same.
#[derive(Debug, Copy, Clone)]
pub struct SampleRateRange {
    pub min: i32,
    pub max: i32,
}

impl From<raw::SoundIoSampleRateRange> for SampleRateRange {
    fn from(range: raw::SoundIoSampleRateRange) -> SampleRateRange {
        SampleRateRange {
            min: range.min,
            max: range.max,
        }
    }
}

impl From<SampleRateRange> for raw::SoundIoSampleRateRange {
    fn from(range: SampleRateRange) -> raw::SoundIoSampleRateRange {
        raw::SoundIoSampleRateRange {
            min: range.min,
            max: range.max,
        }
    }
}

/// This is used for reporting software latency, that is the latency not including
/// latency due to hardware. It is returned by `Device::software_latency()`.
#[derive(Debug, Copy, Clone)]
pub struct SoftwareLatency {
    pub min: f64,
    pub max: f64,
    pub current: f64,
}
