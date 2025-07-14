use bevy::prelude::*;
use rand::{Rng, rng};

use crate::params::{Distances, Weights};

const FORCE_LIMIT: f32 = 0.2;
const VELOCITY_LIMIT: f32 = 4.0;

#[derive(Component, Clone)]
pub struct Boid {
	pub id: u16,
	pub velocity: Vec2,
	pub acceleration: Vec2,
	pub color: Color,
}

impl Boid {
	pub fn create(id: u16, bounds: &Rect) -> (Boid, Transform) {
		let position = Vec2::new(
			rng().random_range(bounds.min.x..bounds.max.x),
			rng().random_range(bounds.min.y..bounds.max.y),
		);

		let boid = Boid {
			id,
			velocity: Vec2::new(rng().random_range(-1.0..1.0), rng().random_range(-1.0..1.0))
				* VELOCITY_LIMIT,
			acceleration: Vec2::ZERO,
			color: Color::srgb(
				rng().random_range(0.3..1.0),
				rng().random_range(0.3..1.0),
				rng().random_range(0.3..1.0),
			),
		};

		let transform = Transform::from_translation(position.extend(0.0));
		(boid, transform)
	}

	pub fn update(
		&mut self,
		transform: &mut Transform,
		neighbors: &[(&Boid, &Transform)],
		distances: &Distances,
		weights: &Weights,
		attractors: &[Vec2],
		bounds: &Rect,
	) {
		let position = transform.translation.truncate();
		self.wrap(transform, bounds);

		let mut seek = attractors.iter().fold(Vec2::ZERO, |acc, attractor| {
			let distance = attractor.distance(position);
			if distance > 0.0 {
				acc + (*attractor - position) / distance
			} else {
				acc
			}
		});

		if !attractors.is_empty() {
			seek /= attractors.len() as f32;
		}

		let mut align = Vec2::ZERO;
		let mut cohere = Vec2::ZERO;
		let mut disperse = Vec2::ZERO;
		let mut cohere_count = 0.0;

		for (other, other_transform) in neighbors {
			if other.id == self.id {
				continue;
			}

			let other_position = other_transform.translation.truncate();
			let distance = other_position.distance(position);

			if distance < distances.align {
				align += other.velocity;
			}

			if distance < distances.cohere {
				cohere += other_position;
				cohere_count += 1.0;
			}

			if distance < distances.disperse && distance > 0.0 {
				disperse += (position - other_position) / distance.powi(2);
			}
		}

		if cohere_count > 0.0 {
			cohere = (cohere / cohere_count) - position;
		}

		self.acceleration = (self.normalize_steering_vector(seek) * weights.seek)
			+ (self.normalize_steering_vector(align) * weights.align)
			+ (self.normalize_steering_vector(cohere) * weights.cohere)
			+ (self.normalize_steering_vector(disperse) * weights.disperse);

		self.velocity = (self.velocity + self.acceleration).clamp_length_max(VELOCITY_LIMIT);
		transform.translation += self.velocity.extend(0.0);
	}

	fn wrap(&mut self, transform: &mut Transform, bounds: &Rect) {
		let mut position = transform.translation.truncate();

		if position.x > bounds.max.x {
			position.x = bounds.min.x;
		} else if position.x < bounds.min.x {
			position.x = bounds.max.x;
		}

		if position.y > bounds.max.y {
			position.y = bounds.min.y;
		} else if position.y < bounds.min.y {
			position.y = bounds.max.y;
		}

		transform.translation = position.extend(0.0);
	}

	fn normalize_steering_vector(&self, v: Vec2) -> Vec2 {
		if v.length() == 0.0 {
			return v;
		}

		let target = match v.try_normalize() {
			Some(vec) => vec * VELOCITY_LIMIT,
			None => v,
		};

		(target - self.velocity).clamp_length_max(FORCE_LIMIT)
	}
}

impl PartialEq for Boid {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
