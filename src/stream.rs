use std::os::raw::c_int;
use std::rc::Rc;

use crate::device::Device;
use crate::error::Error;
use crate::format::{Format, HasDefaultFormat};
use crate::layout::{ChannelLayout, HasDefaultLayout};

pub type Callback = Box<dyn FnMut()>;

pub enum SampleRate {
    Exact(i32),
    NearestTo(i32),
}

pub struct StreamOptions<Frame: sample::Frame> {
    pub sample_rate: SampleRate,
    pub format: Format,
    pub desired_frames_per_buffer: Option<i32>,

    layout: ChannelLayout,
    phantom: std::marker::PhantomData<Frame>,
}

impl<Frame, Sample> Default for StreamOptions<Frame>
where
    Frame: sample::Frame<Sample = Sample> + HasDefaultLayout,
    Sample: sample::Sample + HasDefaultFormat,
{
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(soundio::StreamOptions::<[f32; 2]>::default().format, soundio::Format::Float32LE)
    /// ```
    fn default() -> StreamOptions<Frame> {
        StreamOptions::new(SampleRate::NearestTo(44100), Sample::DEFAULT_FORMAT)
    }
}

impl<Frame: sample::Frame> StreamOptions<Frame> {
    /// Returns a `StreamOptions` with the given sample rate and format.
    ///
    /// Use this method if the frame type does not have a default frame and layout (see
    /// `HasDefaultFormat`).
    ///
    /// # Examples
    ///
    /// ```
    /// let options = soundio::StreamOptions::<[sample::I24; 2]>::new(
    ///         soundio::SampleRate::NearestTo(44100),
    ///         soundio::Format::S24LE
    /// );
    /// ```
    pub fn new(sample_rate: SampleRate, format: Format) -> StreamOptions<Frame> {
        StreamOptions {
            sample_rate,
            format,
            layout: crate::layout::ChannelLayout::get_builtin(
                crate::layout::ChannelLayoutId::Stereo,
            ),
            desired_frames_per_buffer: None,
            phantom: std::marker::PhantomData,
        }
    }
}

pub struct OutStream {
    pub raw: *mut raw::SoundIoOutStream,
    pub callback: Callback,
    pub parent_device: Rc<Device>,
}

pub extern "C" fn outstream_write_callback(
    stream: *mut raw::SoundIoOutStream,
    frame_count_min: c_int,
    frame_count_max: c_int,
) {
    // // Use stream.userdata to get a reference to the OutStreamUserData object.
    // let outstream = unsafe { (*stream).userdata as *mut OutStream };
    // let outstream = outstream.as_mut().unwrap();

    // let stream_writer = OutStreamWriter {
    //     outstream: outstream,
    //     frame_count_min: frame_count_min as _,
    //     frame_count_max: frame_count_max as _,
    //     write_started: false,
    //     channel_areas: Vec::new(),
    //     frame_count: 0,
    //     phantom: PhantomData,
    // };

    // (outstream.write_callback)(&mut stream_writer);
}

pub extern "C" fn outstream_underflow_callback(stream: *mut raw::SoundIoOutStream) {
    // Use stream.userdata to get a reference to the OutStreamUserData object.
    // let raw_userdata_pointer = unsafe { (*stream).userdata as *mut OutStreamUserData };
    // let userdata = unsafe { &mut (*raw_userdata_pointer) };

    // if let Some(ref mut cb) = userdata.underflow_callback {
    //     cb();
    // } else {
    //     println!("Underflow!");
    // }
}

pub extern "C" fn outstream_error_callback(stream: *mut raw::SoundIoOutStream, err: c_int) {
    // Use stream.userdata to get a reference to the OutStreamUserData object.
    // let raw_userdata_pointer = unsafe { (*stream).userdata as *mut OutStreamUserData };
    // let userdata = unsafe { &mut (*raw_userdata_pointer) };

    // if let Some(ref mut cb) = userdata.error_callback {
    //     cb(err.into());
    // } else {
    //     println!("Error: {}", Error::from(err));
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::format::native;

    #[test]
    fn test_default_format() {
        assert_eq!(
            StreamOptions::<[f32; 2]>::default().format,
            native::Float32NE
        );
    }
}
