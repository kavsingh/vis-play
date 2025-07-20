use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::math::Rect;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::{Rng, rng};

use crate::params;
use crate::spatial_grid::SpatialGrid;

static WINDOW_WIDTH: f32 = 1024.0;
static WINDOW_HEIGHT: f32 = 768.0;
static FORCE_LIMIT: f32 = 0.2;
static VELOCITY_LIMIT: f32 = 4.0;

#[derive(Component, Clone, Copy)]
pub struct Movement {
	pub position: Vec2,
	pub velocity: Vec2,
	pub acceleration: Vec2,
}

#[derive(Component)]
struct Boid;

#[derive(Bundle)]
struct BoidBundle {
	marker: Boid,
	movement: Movement,
	transform: Transform,
	sprite: Sprite,
}

// Resources
#[derive(Resource)]
struct FlockingParams {
	pub distances: params::Distances,
	pub weights: params::Weights,
}

#[derive(Resource)]
struct Attractors {
	pub positions: Vec<Vec2>,
}

#[derive(Resource)]
struct World {
	grid: SpatialGrid,
	bounds: Rect,
}

pub fn run() {
	let distances = params::Distances::default();
	let cell_size = distances.mean();

	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "vis-rs".to_string(),
				resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
				canvas: Some("canvas#vis-rs".to_string()),
				..Default::default()
			}),
			..Default::default()
		}))
		.insert_resource(ClearColor(Color::BLACK))
		.insert_resource(FlockingParams {
			distances,
			weights: params::Weights::default(),
		})
		.insert_resource(Attractors {
			positions: Vec::new(),
		})
		.insert_resource(World {
			grid: SpatialGrid::new(cell_size),
			bounds: Rect::new(
				-WINDOW_WIDTH / 2.0,
				-WINDOW_HEIGHT / 2.0,
				WINDOW_WIDTH / 2.0,
				WINDOW_HEIGHT / 2.0,
			),
		})
		.add_systems(Startup, setup)
		.add_systems(Update, (update_world, update_boids).chain())
		.run();
}

fn setup(world: Res<World>, mut commands: Commands) {
	commands.spawn(Camera2d);

	let count = if cfg!(debug_assertions) { 300 } else { 4_000 };

	for _ in 0..count {
		let position = Vec2 {
			x: rng().random_range(world.bounds.min.x..world.bounds.max.x),
			y: rng().random_range(world.bounds.min.y..world.bounds.max.y),
		};
		let velocity = Vec2 {
			x: rng().random_range(-1.0..1.0),
			y: rng().random_range(-1.0..1.0),
		} * VELOCITY_LIMIT;
		let mut transform = Transform::from_translation(position.extend(0.0));

		transform.rotation = Quat::from_rotation_z(velocity.y.atan2(velocity.x));

		commands.spawn(BoidBundle {
			marker: Boid,
			movement: Movement {
				position,
				velocity,
				acceleration: Vec2::new(-0.1, -0.1),
			},
			transform,
			sprite: Sprite {
				color: Color::oklch(
					rng().random_range(0.8..1.0),
					rng().random_range(0.0..1.0),
					rng().random_range(0.0..1.0),
				),
				custom_size: Some(Vec2::new(6.0, 2.0)),
				..Default::default()
			},
		});
	}
}

fn update_world(
	params: Res<FlockingParams>,
	mut world: ResMut<World>,
	query: Query<(Entity, &Boid, &Movement)>,
) {
	world.grid.reset(Some(params.distances.mean()));

	for (entity, _, movement) in query.iter() {
		world.grid.insert(entity, *movement);
	}
}

fn update_boids(
	params: Res<FlockingParams>,
	world: Res<World>,
	attractors: Res<Attractors>,
	mut query: Query<(Entity, &Boid, &mut Movement, &mut Transform)>,
) {
	for (entity, _, mut movement, mut transform) in query.iter_mut() {
		mut_wrap_prosition(&mut movement, &world.bounds);

		let mut seek = attractors
			.positions
			.iter()
			.fold(Vec2::ZERO, |acc, attractor| {
				let distance = attractor.distance(movement.position);
				if distance > 0.0 {
					acc + (*attractor - movement.position) / distance
				} else {
					acc
				}
			});

		if !attractors.positions.is_empty() {
			seek /= attractors.positions.len() as f32;
		}

		let mut align = Vec2::ZERO;
		let mut cohere = Vec2::ZERO;
		let mut disperse = Vec2::ZERO;
		let mut cohere_count = 0.0;

		let neighbors = world
			.grid
			.get_neighbors(&movement.position, params.distances.max());

		for (other, other_movement, distance) in neighbors {
			if other == entity {
				continue;
			}

			if distance < params.distances.align {
				align += other_movement.velocity;
			}

			if distance < params.distances.cohere {
				cohere += other_movement.position;
				cohere_count += 1.0;
			}

			if distance < params.distances.disperse && distance > 0.0 {
				disperse += (movement.position - other_movement.position) / distance.powi(2);
			}
		}

		if cohere_count > 0.0 {
			cohere = (cohere / cohere_count) - movement.position;
		}

		movement.acceleration = (normalize_steering_vector(movement.velocity, seek)
			* params.weights.seek)
			+ (normalize_steering_vector(movement.velocity, align) * params.weights.align)
			+ (normalize_steering_vector(movement.velocity, cohere) * params.weights.cohere)
			+ (normalize_steering_vector(movement.velocity, disperse) * params.weights.disperse);
		movement.velocity =
			(movement.velocity + movement.acceleration).clamp_length_max(VELOCITY_LIMIT);

		let vel = movement.velocity;

		movement.position += vel;
		transform.rotation = Quat::from_rotation_z(vel.y.atan2(vel.x));
		transform.translation = movement.position.extend(0.0);
	}
}

fn mut_wrap_prosition(movement: &mut Mut<Movement>, bounds: &Rect) {
	if movement.position.x > bounds.max.x {
		movement.position.x = bounds.min.x;
	} else if movement.position.x < bounds.min.x {
		movement.position.x = bounds.max.x;
	}

	if movement.position.y > bounds.max.y {
		movement.position.y = bounds.min.y;
	} else if movement.position.y < bounds.min.y {
		movement.position.y = bounds.max.y;
	}
}

fn normalize_steering_vector(from: Vec2, towards: Vec2) -> Vec2 {
	if towards.length() == 0.0 {
		return towards;
	}

	let normalized_towards = match towards.try_normalize() {
		Some(vec) => vec * VELOCITY_LIMIT,
		None => from,
	};

	(normalized_towards - from).clamp_length_max(FORCE_LIMIT)
}
