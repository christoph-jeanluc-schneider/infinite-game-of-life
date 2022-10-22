#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{IconData, NativeOptions};
use infinite_game_of_life::app::*;
use rgb::*;

fn main() {
	let image = lodepng::decode32_file("assets/chart-grid-dots.png").unwrap();
	let icon = IconData {
		rgba: image.buffer.as_bytes().to_vec(),
		width: image.width as u32,
		height: image.height as u32,
	};
	let options = NativeOptions {
		icon_data: Some(icon),
		..Default::default()
	};
	eframe::run_native(
		"Infinite Game of Life",
		options,
		Box::new(|_cc| Box::new(App::default())),
	);
}
