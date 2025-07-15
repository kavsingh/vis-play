#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod boid;
mod grid;
mod params;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub async fn vis() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	crate::app::run_app().await;
}
