extern crate soundio;
extern crate crossbeam;
extern crate hound;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::BufReader;
use std::fs::File;
use std::env;

// Maybe the best way to do this is something like:
//
// let (write_callback, wav_player) = WavPlayer::new();
//
// Internally they can use a mutex to communicate.
struct WavPlayer {
	reader: hound::WavReader<BufReader<File>>,
	finished: bool,
}

impl WavPlayer {
	fn write_callback(&mut self, stream: &mut soundio::OutStreamWriter) {
		let mut frames_left = stream.frame_count_max();
		let was_finished = self.finished;
		loop {
			if let Err(e) = stream.begin_write(frames_left) {
				println!("Error writing to stream: {}", e);
				return;
			}
			// Hound's sample conversion is not as awesome as mine. This will fail on floating point types.
			let mut s = self.reader.samples::<i32>();

			for f in 0..stream.frame_count() {
	    		for c in 0..stream.channel_count() {
					match s.next() {
						Some(x) => {
							stream.set_sample(c, f, x.unwrap()*1000); 
						},
						None => {
							stream.set_sample(c, f, 0);
							self.finished = true;
						}
					}
					
				}
			}

			frames_left -= stream.frame_count();
			if frames_left <= 0 {
				break;
			}

			stream.end_write();
		}
		if self.finished != was_finished {
	//		stream.wakeup();
		}
	}

	fn finished(&self) -> bool {
		self.finished
	}
}

// TODO: I need some interior mutability and a mutex to make the write_callback work nicely.

// Print sound soundio debug info and play back a sound.
fn play(filename: &str) -> Result<(), String> {
	// Try to open the file.
	let reader = hound::WavReader::open(filename).map_err(|x| x.to_string())?;
	
	println!("Soundio version: {}", soundio::version_string());

	let mut ctx = soundio::Context::new();
	ctx.set_app_name("Player");
	ctx.connect()?;

	println!("Current backend: {:?}", ctx.current_backend());

	// We have to flush events so we can scan devices.
	println!("Flushing events.");
	ctx.flush_events();
	println!("Flushed");

	let channels = reader.spec().channels;
	let sample_rate = reader.spec().sample_rate;
	let int_or_float = reader.spec().sample_format;
	let bits_per_sample = reader.spec().bits_per_sample;

	// I guess these are always signed little endian?
	let soundio_format = match int_or_float {
		hound::SampleFormat::Int => match bits_per_sample {
				8 => soundio::Format::S8,
				16 => soundio::Format::S16LE,
				24 => soundio::Format::S24LE,
				32 => soundio::Format::S32LE,
				_ => return Err(format!("Unknown bit depth: {}", bits_per_sample)),
			},

		hound::SampleFormat::Float => match bits_per_sample {
				32 => soundio::Format::Float32LE,
				64 => soundio::Format::Float64LE,
				_ => return Err(format!("Unknown bit depth: {}", bits_per_sample)),
			},
	};

	let default_layout = soundio::ChannelLayout::get_default(channels as _);
	println!("Default layout for {} channel(s): {:?}", channels, default_layout);

	let output_dev = ctx.default_output_device().map_err(|_| "Error getting default output device".to_string())?;

	println!("Default output device: {} {}", output_dev.name(), if output_dev.is_raw() { "raw" } else { "cooked" } );

	let mut player = WavPlayer {
		reader: reader,
		finished: false,
	};

	println!("Opening default output stream");
	let mut output_stream = output_dev.open_outstream(
		sample_rate as _,
		soundio_format,
		default_layout,
		2.0,
		|x| player.write_callback(x), // The trouble is this borrows &mut player, so I can't use it at all elsewhere. It's correct because player can be mutated. But I still want to read a value of it. The only solution is interior mutability.
		None::<fn()>,
		None::<fn(soundio::Error)>,
	)?;

	println!("Starting stream");
	output_stream.start()?;

	// Wait for key presses.
	println!("Press enter to stop playback");
	let stdin = io::stdin();
	let input = &mut String::new();
	let _ = stdin.read_line(input);

	Ok(())
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Usage: {} <filename.wav>", args[0]);
		return;
	}


	match play(&args[1]) {
		Err(x) => println!("Error: {}", x),
		_ => {},
	}
}