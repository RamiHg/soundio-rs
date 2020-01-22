extern crate soundio;
extern crate crossbeam;

use std::f64::consts::PI;
use std::io;

struct SineWavePlayer {
	phase: f64, // Phase is updated each time the write callback is called.
	frequency: f64,
	amplitude: f64, // TODO: For some reason amplitude close to 1 (maybe > 0.99?) and high frequency (e.g. 18 kHz) gives weird low frequency aliasing or something.
}

impl SineWavePlayer {
	fn write_callback(&mut self, stream: &mut soundio::OutStreamWriter) {
		let mut frames_left = stream.frame_count_max();

		loop {
			if let Err(e) = stream.begin_write(frames_left) {
				println!("Error writing to stream: {}", e);
				return;
			}
			let phase_step = self.frequency / stream.sample_rate() as f64 * 2.0 * PI;

			for c in 0..stream.channel_count() {
				for f in 0..stream.frame_count() {
					stream.set_sample(c, f, (self.phase.sin() * self.amplitude) as f32);
					self.phase += phase_step;
				}
			}

			frames_left -= stream.frame_count();
			if frames_left <= 0 {
				break;
			}

			stream.end_write();
		}
	}
}

// Print sound soundio debug info and play back a sound.
fn run() -> Result<(), String> {

	println!("Soundio version: {}", soundio::version_string());

	let (major, minor, patch) = soundio::version();

	println!("Major version: {}, minor version: {}, patch version: {}", major, minor, patch);

	let backend_list = [
		soundio::Backend::Jack,
		soundio::Backend::PulseAudio,
		soundio::Backend::Alsa,
		soundio::Backend::CoreAudio,
		soundio::Backend::Wasapi,
		soundio::Backend::Dummy,
	];

	for &backend in backend_list.iter() {
		println!("Backend {} available? {}", backend, soundio::have_backend(backend));
	} 

	println!("InitAudioBackend error: {}", soundio::Error::InitAudioBackend);

	let mut ctx = soundio::Context::new();

	ctx.set_app_name("Sine Wave");

	println!("Available backends: {:?}", ctx.available_backends());

	ctx.connect()?;

	println!("Current backend: {:?}", ctx.current_backend());

	// We have to flush events so we can scan devices.
	ctx.flush_events();

	// Builtin and default layouts.

	let builtin_layouts = soundio::ChannelLayout::get_all_builtin();
	for layout in builtin_layouts {
		println!("Builtin layout: {:?}", layout);
	}

	let default_mono_layout = soundio::ChannelLayout::get_default(1);
	println!("Default mono layout: {:?}", default_mono_layout);
	let default_stereo_layout = soundio::ChannelLayout::get_default(2);
	println!("Default stereo layout: {:?}", default_stereo_layout);


	println!("Input device count: {}", ctx.input_device_count());
	println!("Output device count: {}", ctx.output_device_count());

	let output_devices = ctx.output_devices().map_err(|_| "Error getting output devices".to_string())?;
	let input_devices = ctx.input_devices().map_err(|_| "Error getting input devices".to_string())?;

	for dev in output_devices {
		println!("Output device: {} {}", dev.name(), if dev.is_raw() { "raw" } else { "cooked" } );
	}

	for dev in input_devices {
		println!("Input device: {} {}", dev.name(), if dev.is_raw() { "raw" } else { "cooked" } );
	}

	let output_dev = ctx.default_output_device().map_err(|_| "Error getting default output device".to_string())?;

	println!("Default output device: {} {}", output_dev.name(), if output_dev.is_raw() { "raw" } else { "cooked" } );

	let mut sine = SineWavePlayer {
		phase: 0.0,
		amplitude: 0.3,
		frequency: 200.0,
	};

	println!("Opening default output stream");
	let mut output_stream = output_dev.open_outstream(
		48000,
		soundio::Format::Float32LE,
		soundio::ChannelLayout::get_default(2),
		0.5,
		move |x| sine.write_callback(x),
		None::<fn()>,
		None::<fn(soundio::Error)>,
	)?;

	println!("Starting stream");
	output_stream.start()?;

	// Run the loop in a new thread.
	println!("Press enter to exit");
	let stdin = io::stdin();
	let input = &mut String::new();
	let _ = stdin.read_line(input);

	// Wait for key presses.
	Ok(())
}

fn main() {
	match run() {
		Err(x) => println!("Error: {}", x),
		_ => {},
	}
}