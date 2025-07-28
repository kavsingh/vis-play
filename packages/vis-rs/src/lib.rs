#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod boids;
mod params;
mod spatial_grid;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn vis(count: i32) {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	crate::boids::run(count);
}
