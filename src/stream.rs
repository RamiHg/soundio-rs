use std::os::raw::c_int;
use std::rc::Rc;

use crate::device::Device;
use crate::error::{self, Error, Result};
use crate::format::{Format, HasDefaultFormat};
use crate::layout::{ChannelLayout, HasDefaultLayout};

pub type BoxedCallback<Frame> = Box<dyn FnMut(&mut [Frame])>;

#[derive(Clone)]
pub enum SampleRate {
    Exact(i32),
    NearestTo(i32),
}

#[derive(Clone)]
pub struct StreamOptions<Frame> {
    pub sample_rate: SampleRate,
    pub format: Format,
    pub desired_frames_per_buffer: Option<i32>,

    pub layout: ChannelLayout,
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

impl<Frame> StreamOptions<Frame> {
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

pub struct OutStream<Frame: sample::Frame> {
    pub raw: *mut raw::SoundIoOutStream,
    pub callback: BoxedCallback<Frame>,
    pub parent_device: Rc<Device>,
    pub options: StreamOptions<Frame>,
}

impl<Frame: sample::Frame> Drop for OutStream<Frame> {
    fn drop(&mut self) {
        unsafe { raw::soundio_outstream_destroy(self.raw) }
    }
}

impl<Frame: sample::Frame> OutStream<Frame> {
    pub fn start(&self) -> Result<()> {
        unsafe { error::from_code(raw::soundio_outstream_start(self.raw)) }
    }
}

pub extern "C" fn outstream_write_callback<Frame: sample::Frame>(
    raw: *mut raw::SoundIoOutStream,
    frame_count_min: c_int,
    frame_count_max: c_int,
) {
    let outstream = unsafe { ((*raw).userdata as *mut OutStream<Frame>).as_mut().unwrap() };
    let mut sound_areas = std::ptr::null_mut();
    let mut frame_count = (outstream.options.desired_frames_per_buffer.unwrap_or(0) as c_int)
        .max(frame_count_min)
        .min(frame_count_max);
    match unsafe { raw::soundio_outstream_begin_write(raw, &mut sound_areas, &mut frame_count) } {
        0 => (),
        x => panic!("{:?}", Error::from(x)),
    };
    let sound_areas = unsafe { std::slice::from_raw_parts_mut(sound_areas, Frame::n_channels()) };
    // Make sure that the frame stride is what we think it is.
    assert_eq!(
        sound_areas[0].step,
        (std::mem::size_of::<Frame::Sample>() * Frame::n_channels()) as i32
    );
    // Finally, convert to a buffer of our frame type.
    let buffer: &mut [Frame] = unsafe {
        std::slice::from_raw_parts_mut(
            sound_areas[0].ptr as *mut _,
            frame_count as usize * Frame::n_channels(),
        )
    };
    // Call the callback.
    (outstream.callback)(buffer);
    match unsafe { raw::soundio_outstream_end_write(raw) } {
        0 => (),
        x => panic!("{:?}", Error::from(x)),
    };
}

pub extern "C" fn outstream_underflow_callback(stream: *mut raw::SoundIoOutStream) {
    // TODO: Do we care about underflow?
}

pub extern "C" fn outstream_error_callback(stream: *mut raw::SoundIoOutStream, err: c_int) {
    // TODO: Do we care about error handling?
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::context::Context;
    use crate::device::Device;

    use crate::format::native;

    fn dummy(_buffer: &mut [[f32; 2]]) {}

    #[test]
    fn test_default_format() {
        assert_eq!(
            StreamOptions::<[f32; 2]>::default().format,
            native::Float32NE
        );
    }

    #[test]
    fn test_opens_outstream() -> Result<()> {
        let context = Context::default();
        context.connect()?;
        let device = context.default_output_device()?;
        let stream =
            device.open_outstream(StreamOptions::<[f32; 2]>::default(), Box::new(dummy))?;
        stream.start()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(())
    }
}
