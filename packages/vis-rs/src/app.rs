use std::cell::RefCell;

use nannou::color::{Laba, white_point};
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

use crate::boid::Boid;
use crate::grid::SpatialGrid;
use crate::params::{Distances, Weights};

pub struct Model {
	flock: Vec<Boid>,
	grid: SpatialGrid,
	distances: Distances,
	weights: Weights,
	attractors: Vec<Point2>,
	bg_color: Laba<white_point::D65>,
	show_debug: bool,
}

impl Default for Model {
	fn default() -> Self {
		Self {
			flock: vec![],
			grid: SpatialGrid::new(50.0), // Cell size slightly larger than largest interaction radius
			distances: Distances::default(),
			weights: Weights::default(),
			attractors: vec![],
			bg_color: Laba::new(0.0, 0.0, 0.0, 1.0),
			show_debug: false,
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
				let count = if cfg!(debug_assertions) { 400 } else { 4_000 };

				for id in 0..count {
					app_model.flock.push(Boid::create(id, &bounds));
				}

				app_model
			})
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.update(update)
	.event(event)
	.run_async()
	.await;
}

fn update(app: &App, model: &mut Model, _update: Update) {
	let bounds = app.window_rect();

	// Update spatial grid
	model.grid.clear();
	for boid in model.flock.iter().cloned() {
		model.grid.insert(boid);
	}

	// Update each boid using nearby neighbors
	let max_radius = model
		.distances
		.cohere
		.max(model.distances.align)
		.max(model.distances.disperse);

	for boid in model.flock.iter_mut() {
		let neighbors = model.grid.get_neighbors(boid.position(), max_radius);
		boid.update(
			&neighbors,
			&model.distances,
			&model.weights,
			&model.attractors,
			&bounds,
		);
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
	let draw = app.draw();
	let bounds = app.window_rect();

	// Draw background
	draw.rect()
		.xy(frame.rect().xy())
		.wh(frame.rect().wh())
		.color(model.bg_color);

	// Draw grid debug visualization if enabled
	if model.show_debug {
		model.grid.draw_debug(&draw, &bounds);
	}

	#[cfg(debug_assertions)]
	{
		use nannou::color::Lab;

		model.attractors.iter().for_each(|attractor| {
			draw.ellipse()
				.x_y(attractor.x, attractor.y)
				.w_h(3.0, 3.0)
				.color(Lab::new(68.0, -0.21, -48.9));
		});
	}

	// Draw boids on top
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

fn event(_app: &App, model: &mut Model, event: Event) {
	if let Event::WindowEvent {
		id: _,
		simple: Some(WindowEvent::KeyPressed(Key::D)),
	} = event
	{
		model.show_debug = !model.show_debug;
	}
}
