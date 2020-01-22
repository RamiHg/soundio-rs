extern crate soundio;

fn devices_changed() {
	println!("Devices changed!");
	// TODO: This is really un-ergonomic. Really I just want wait_events() to return
	// an event, e.g. devices_changed, some_event, whatever.
	//
	// That'd be much nicer. Callbacks suck.
}

fn list_devices(watch: bool, short_output: bool, backend: soundio::Backend) -> Result<(), String> {

	println!("Soundio version: {}", soundio::version_string());

	let mut ctx = if watch {
		soundio::Context::new()
	} else {
		soundio::Context::new_with_callbacks(None::<fn(soundio::Error)>, Some(devices_changed), None::<fn()>)
	};
    ctx.set_app_name("Device Lister");

	if backend == soundio::Backend::None {
		ctx.connect()?;
	} else {
		ctx.connect_backend(backend)?;
	}

	println!("Connected to backend: {:?}", ctx.current_backend());

	// Required before we can open devices.
	ctx.flush_events();

	print_all_devices(&ctx, short_output)?;

	// Wait forever if they tell us to watch for device changes.
	if watch {
		loop {
			ctx.wait_events();
		}
	}

	Ok(())
}

fn print_all_devices(ctx: &soundio::Context, short_output: bool) -> Result<(), String> {
	// Print a list of devices.
	let input_count = ctx.input_device_count();
	let output_count = ctx.output_device_count();

	let default_input = ctx.default_input_device_index();
	let default_output = ctx.default_output_device_index();

	println!("--------Input Devices--------\n");
	for i in 0..input_count {
		print_device(&ctx.input_device(i)?, default_input.map_or(false, |d| d == i), short_output);
	}

	println!("--------Output Devices--------\n");
	for i in 0..output_count {
		print_device(&ctx.output_device(i)?, default_output.map_or(false, |d| d == i), short_output);
	}

	println!("\n{} devices found", input_count + output_count);

	Ok(())
}

fn print_device(device: &soundio::Device, is_default: bool, short_output: bool) {
	println!("\n{}{}{}",
			device.name(),
			if is_default { " (default)" } else { "" },
			if device.is_raw() { " (raw)" } else { "" }
		);

	if short_output {
		return;
	}

	println!("    Channel layouts: {:?}", device.layouts());
	println!("    Current layout: {:?}", device.current_layout());

	println!("    Sample rates: {:?}", device.sample_rates());
	println!("    Current sample rate: {}", device.current_sample_rate());

	println!("    Formats: {:?}", device.formats());
	println!("    Current format: {}", device.current_format());

	println!("    Software latency: {:?}", device.software_latency());
}

fn main() {
 /*   let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Usage: {} <filename.wav>", args[0]);
		return;
	}*/


	match list_devices(false, false, soundio::Backend::None) {
		Err(x) => println!("Error: {}", x),
		_ => {},
	}
}