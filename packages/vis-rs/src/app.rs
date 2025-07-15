use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::{Rng, rng};

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
	);

	app.run();
}

fn setup_system(mut commands: Commands, bounds: Res<WorldBounds>) {
	// Spawn camera
	commands.spawn(Camera2d);

	// Spawn boids
	let count = if cfg!(debug_assertions) { 400 } else { 4_000 };

	for id in 0..count {
		commands.spawn((
			Boid::create(id, &bounds.rect),
			Transform::default(),
			Sprite {
				color: Color::oklch(
					rng().random_range(0.8..1.0),
					rng().random_range(0.0..1.0),
					rng().random_range(0.0..1.0),
				),
				custom_size: Some(Vec2::new(6.0, 2.0)),
				..Default::default()
			},
		));
	}
}

fn update_spatial_grid(mut grid: ResMut<FlockGrid>, query: Query<&Boid>) {
	grid.grid.clear();

	for boid in query.iter() {
		grid.grid.insert(boid);
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
	for (boid, _) in query.iter() {
		let neighbors = grid.grid.get_neighbors(boid.position, max_radius);
		boids_to_update.push((boid.clone(), neighbors));
	}

	// Update boids
	for (mut boid, mut transform) in query.iter_mut() {
		if let Some((_, neighbors)) = boids_to_update.iter().find(|(b, _)| b.id == boid.id) {
			boid.update(
				neighbors,
				&params.distances,
				&params.weights,
				&attractors.positions,
				&bounds.rect,
			);

			transform.translation = boid.position.extend(0.0);
			transform.rotation = Quat::from_rotation_z(boid.velocity.y.atan2(boid.velocity.x));
		}
	}
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut debug: ResMut<DebugSettings>) {
	if keys.just_pressed(KeyCode::KeyD) {
		debug.show_debug = !debug.show_debug;
	}
}
