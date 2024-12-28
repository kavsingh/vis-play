use std::cell::RefCell;

use nannou::color::{white_point, Lab, Laba};
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

use crate::boid::{Boid, Weights};

pub struct Model {
	flock: Vec<Boid>,
	attractors: Vec<Point2>,
	weights: Weights,
	bg_color: Laba<white_point::D65>,
}

impl Default for Model {
	fn default() -> Self {
		Self {
			flock: vec![],
			attractors: vec![],
			weights: Weights::default(),
			bg_color: Laba::new(0.0, 0.0, 0.0, 0.4),
		}
	}
}

pub async fn run_app(model: Model) {
	thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

	MODEL.with(|m| m.borrow_mut().replace(model));

	app::Builder::new_async(|app| {
		Box::new(async move {
			create_window(app).await;
			MODEL.with(|m| {
				let mut app_model = m.borrow_mut().take().unwrap();
				let bounds = app.window_rect();
				let count = if cfg!(debug_assertions) { 200 } else { 1000 };

				for _ in 0..count {
					app_model.flock.push(Boid::create(&bounds));
				}

				app_model
			})
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.update(update)
	.run_async()
	.await;
}

fn update(app: &App, model: &mut Model, _update: Update) {
	let flock = model.flock.clone();
	let bounds = app.window_rect();

	model
		.flock
		.iter_mut()
		.for_each(|boid| boid.update(&flock, &model.attractors, &model.weights, &bounds));
}

fn view(app: &App, model: &Model, frame: Frame) {
	let draw = app.draw();

	draw.rect()
		.x_y(frame.rect().x(), frame.rect().y())
		.wh(frame.rect().wh())
		.color(model.bg_color);

	model.flock.iter().for_each(|boid| boid.draw(&draw));

	#[cfg(debug_assertions)]
	model.attractors.iter().for_each(|attractor| {
		draw.ellipse()
			.x_y(attractor.x, attractor.y)
			.w_h(3.0, 3.0)
			.color(Lab::new(68.0, -0.21, -48.9));
	});

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
		.title("vis-rs")
		.view(view)
		.build_async()
		.await
		.unwrap();
}
