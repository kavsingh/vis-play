use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::boid::Boid;
use crate::grid::SpatialGrid;
use crate::params::{Distances, Weights};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;

// Resources
#[derive(Resource)]
pub struct FlockingParams {
	pub distances: Distances,
	pub weights: Weights,
}

#[derive(Resource)]
pub struct Attractors {
	pub positions: Vec<Vec2>,
}

#[derive(Resource)]
pub struct WorldBounds {
	pub rect: Rect,
}

#[derive(Resource)]
pub struct FlockGrid {
	pub grid: SpatialGrid,
}

#[derive(Resource)]
pub struct DebugSettings {
	pub show_debug: bool,
}

pub async fn run_app() {
	let mut app = App::new();

	app.add_plugins(DefaultPlugins.set(WindowPlugin {
		primary_window: Some(Window {
			title: "vis-rs".to_string(),
			resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
			canvas: Some("#vis-rs".to_string()),
			..Default::default()
		}),
		..Default::default()
	}))
	.insert_resource(FlockingParams {
		distances: Distances::default(),
		weights: Weights::default(),
	})
	.insert_resource(Attractors {
		positions: Vec::new(),
	})
	.insert_resource(WorldBounds {
		rect: Rect::new(
			-WINDOW_WIDTH / 2.0,
			-WINDOW_HEIGHT / 2.0,
			WINDOW_WIDTH / 2.0,
			WINDOW_HEIGHT / 2.0,
		),
	})
	.insert_resource(FlockGrid {
		grid: SpatialGrid::new(50.0),
	})
	.insert_resource(DebugSettings { show_debug: false })
	.insert_resource(ClearColor(Color::BLACK))
	.add_systems(Startup, setup_system)
	.add_systems(
		Update,
		(update_spatial_grid, update_boids, handle_input).chain(),
	)
	.add_systems(PostUpdate, render_boids);

	app.run();
}

fn setup_system(mut commands: Commands, bounds: Res<WorldBounds>) {
	// Spawn camera
	commands.spawn(Camera2d);

	// Spawn boids
	let count = if cfg!(debug_assertions) { 400 } else { 2000 };

	for id in 0..count {
		let (boid, transform) = Boid::create(id, &bounds.rect);
		commands.spawn((
			boid,
			transform,
			Sprite {
				color: Color::WHITE,
				custom_size: Some(Vec2::new(6.0, 2.0)),
				..Default::default()
			},
		));
	}
}

fn update_spatial_grid(mut grid: ResMut<FlockGrid>, query: Query<(&Boid, &Transform)>) {
	grid.grid.clear();

	for (boid, transform) in query.iter() {
		grid.grid.insert(boid.clone(), *transform);
	}
}

fn update_boids(
	mut query: Query<(&mut Boid, &mut Transform)>,
	grid: Res<FlockGrid>,
	params: Res<FlockingParams>,
	attractors: Res<Attractors>,
	bounds: Res<WorldBounds>,
) {
	let max_radius = params
		.distances
		.cohere
		.max(params.distances.align)
		.max(params.distances.disperse);

	// Collect all boids for neighbor lookup
	let mut boids_to_update = Vec::new();
	for (boid, transform) in query.iter() {
		let position = transform.translation.truncate();
		let neighbors = grid.grid.get_neighbors(position, max_radius);
		boids_to_update.push((boid.clone(), *transform, neighbors));
	}

	// Update boids
	for (mut boid, mut transform) in query.iter_mut() {
		if let Some((_, _, neighbors)) = boids_to_update.iter().find(|(b, _, _)| b.id == boid.id) {
			boid.update(
				&mut transform,
				neighbors,
				&params.distances,
				&params.weights,
				&attractors.positions,
				&bounds.rect,
			);
		}
	}
}

fn render_boids(mut query: Query<(&Boid, &Transform, &mut Sprite)>) {
	for (boid, _transform, mut sprite) in query.iter_mut() {
		sprite.color = boid.color;
		sprite.custom_size = Some(Vec2::new(6.0, 2.0));

		// Note: In a full implementation, we'd set rotation on the Transform
		// based on velocity direction for proper boid orientation
	}
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut debug: ResMut<DebugSettings>) {
	if keys.just_pressed(KeyCode::KeyD) {
		debug.show_debug = !debug.show_debug;
	}
}
