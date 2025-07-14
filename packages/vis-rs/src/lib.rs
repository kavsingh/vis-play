#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod boid;
mod grid;
mod params;

use wasm_bindgen::prelude::*;

use crate::app::run_app;

#[wasm_bindgen]
pub async fn main_web() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	run_app().await;
}
