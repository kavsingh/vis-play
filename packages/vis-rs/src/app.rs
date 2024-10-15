use std::cell::RefCell;

use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

pub struct Model;

pub async fn run_app(model: Model) {
	thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

	MODEL.with(|m| m.borrow_mut().replace(model));

	app::Builder::new_async(|app| {
		Box::new(async move {
			create_window(app).await;
			MODEL.with(|m| m.borrow_mut().take().unwrap())
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.update(update)
	.run_async()
	.await;
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
	let win = app.window_rect();
	let draw = app.draw();

	draw.background().color(BLACK);

	let radius = win.w().min(win.h()) * 0.25;

	draw.ellipse()
		.x(0.0)
		.radius(radius)
		.color(rgba(1.0, 1.0, 1.0, app.time.sin()));
	draw.to_frame(app, &frame).unwrap();
}

async fn create_window(app: &App) {
	app.new_window()
		.device_descriptor(DeviceDescriptor {
			limits: Limits {
				max_texture_dimension_2d: 8192,
				..Limits::downlevel_webgl2_defaults()
			},
			..Default::default()
		})
		.title("nannou vis")
		.view(view)
		.build_async()
		.await
		.unwrap();
}
