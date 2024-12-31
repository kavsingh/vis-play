#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod boid;
mod params;

use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::app::{run_app, Model};

#[wasm_bindgen]
pub async fn main_web() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	let model = Model::default();

	block_on(async {
		run_app(model).await;
	});
}
