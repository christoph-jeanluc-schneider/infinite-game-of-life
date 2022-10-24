#[derive(Debug)]
pub struct Universe {
	pub cells: Vec<u8>,
}

impl Universe {
	pub fn new(width: u32, height: u32) -> Self {
		let cells: Vec<u8> = (0..width * height)
			.map(|_| if fastrand::bool() { u8::MAX } else { 0 })
			.collect();
		Self { cells }
	}
}
