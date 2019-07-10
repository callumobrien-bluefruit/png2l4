use std::{
	env,
	fs::File,
	io::prelude::*,
	vec::Vec
};

fn main() {
	let filename = env::args().nth(1).unwrap();

	let decoder = png::Decoder::new(File::open(&filename).unwrap());
	let (info, mut reader) = decoder.read_info().unwrap();
	let mut buf = vec![0; info.buffer_size()];
	reader.next_frame(&mut buf).unwrap();
	let converted = convert(&buf);

	let out_filename = filename.replace(".png", ".raw");
	let mut out_file = File::create(out_filename).unwrap();
	out_file.write_all(&converted).unwrap();
}

fn convert(bytes: &Vec<u8>) -> Vec<u8> {
	let mut prev = None;
	let mut converted = Vec::new();

	for byte in bytes {
		match prev {
			Some(b) => converted.push(b + byte >> 4),
			None => prev = Some(byte & 0xf)
		};
	}

	converted
}
