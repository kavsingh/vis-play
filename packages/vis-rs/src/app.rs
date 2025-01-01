use std::cell::RefCell;

use nannou::color::{white_point, Lab, Laba};
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

use crate::boid::Boid;
use crate::params::{Distances, Weights};

pub struct Model {
	flock: Vec<Boid>,
	distances: Distances,
	weights: Weights,
	attractors: Vec<Point2>,
	bg_color: Laba<white_point::D65>,
}

impl Default for Model {
	fn default() -> Self {
		Self {
			flock: vec![],
			distances: Distances::default(),
			weights: Weights::default(),
			attractors: vec![],
			bg_color: Laba::new(0.0, 0.0, 0.0, 1.0),
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
				let count = if cfg!(debug_assertions) { 400 } else { 3_000 };

				for id in 0..count {
					app_model.flock.push(Boid::create(id, &bounds));
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

	for boid in model.flock.iter_mut() {
		boid.update(
			&flock,
			&model.distances,
			&model.weights,
			&model.attractors,
			&bounds,
		)
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
	let draw = app.draw();

	draw.rect()
		.xy(frame.rect().xy())
		.wh(frame.rect().wh())
		.color(model.bg_color);

	#[cfg(debug_assertions)]
	model.attractors.iter().for_each(|attractor| {
		draw.ellipse()
			.x_y(attractor.x, attractor.y)
			.w_h(3.0, 3.0)
			.color(Lab::new(68.0, -0.21, -48.9));
	});

	for boid in model.flock.iter() {
		boid.draw(&draw)
	}

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
