use js_sys::Math;
use wasm_bindgen::prelude::*;

const AGEING: u8 = 5;
const MIN_AGE: u8 = AGEING * 3;

#[wasm_bindgen]
pub struct Rules {
	surv: Vec<u8>,
	born: Vec<u8>,
	init_full: bool,
}

#[wasm_bindgen]
impl Rules {
	pub fn new(surv: Vec<u8>, born: Vec<u8>, init_full: bool) -> Self {
		Self { surv, born, init_full }
	}
}

#[wasm_bindgen]
pub struct Universe {
	width: u32,
	height: u32,
	surv: Vec<u8>,
	born: Vec<u8>,
	cells: Vec<u8>,
}

#[wasm_bindgen]
impl Universe {
	pub fn new(width: u32, height: u32, rules: Rules) -> Self {
		let mut cells: Vec<u8> = vec![];
		if rules.init_full {
			cells = (0..width * height)
				.map(|_| if Math::random() < 0.5 { u8::MAX } else { 0 })
				.collect();
		} else {
			let range_col = ((width as f64 * 0.35) as u32, (width as f64 * 0.65) as u32);
			let range_row = ((height as f64 * 0.35) as u32, (height as f64 * 0.65) as u32);
			for row in 0..height {
				for col in 0..width {
					let cell = if Math::random() < 0.5
						&& col > range_col.0 && col < range_col.1
						&& row > range_row.0 && row < range_row.1
					{
						u8::MAX
					} else {
						0
					};
					cells.push(cell);
				}
			}
		}
		Self {
			width,
			height,
			surv: rules.surv,
			born: rules.born,
			cells,
		}
	}

	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}

	fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}

				let neighbor_row = (row + delta_row) % self.height;
				let neighbor_col = (column + delta_col) % self.width;
				let idx = self.get_index(neighbor_row, neighbor_col);
				if self.cells[idx] > MIN_AGE {
					count += 1;
				}
			}
		}
		count
	}

	pub fn tick(&mut self) {
		let mut next = self.cells.clone();

		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_index(row, col);
				let cell = if self.cells[idx] < AGEING {
					0
				} else {
					self.cells[idx] - AGEING
				};
				let neighbors = self.live_neighbor_count(row, col);
				let mut next_cell = cell;

				if cell == 0 && self.born.contains(&neighbors) {
					next_cell = u8::MAX;
				} else if !self.surv.contains(&neighbors) {
					next_cell = 0;
				}

				next[idx] = next_cell;
			}
		}

		self.cells = next;
	}

	pub fn cells(&self) -> *const u8 {
		self.cells.as_ptr()
	}
}
