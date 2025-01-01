use nannou::color::Lab;
use nannou::prelude::*;

use crate::params::{Distances, Weights};

const FORCE_LIMIT: f32 = 0.2;
const VELOCITY_LIMIT: f32 = 4.0;

#[derive(Clone)]
pub struct Boid {
	id: u16,
	position: Point2,
	velocity: Vec2,
	acceleration: Vec2,
	color: Lab,
}

impl Boid {
	pub fn create(id: u16, bounds: &Rect) -> Boid {
		Boid {
			id,
			position: pt2(
				random_range(bounds.left(), bounds.right()),
				random_range(bounds.bottom(), bounds.top()),
			),
			velocity: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)) * VELOCITY_LIMIT,
			acceleration: vec2(0.0, 0.0),
			color: Lab::new(
				random_range(80.0, 100.0),
				random_range(-32.0, 128.0),
				random_range(-32.0, 128.0),
			),
		}
	}

	pub fn update(
		&mut self,
		flock: &[Boid],
		distances: &Distances,
		weights: &Weights,
		attractors: &[Point2],
		bounds: &Rect,
	) {
		self.wrap(bounds);

		let mut seek = attractors.iter().fold(vec2(0.0, 0.0), |acc, attractor| {
			acc + (*attractor - self.position) / (attractor.distance(self.position))
		});

		if attractors.len() > 1 {
			seek /= attractors.len() as f32;
		}

		let mut align = vec2(0.0, 0.0);
		let mut cohere = vec2(0.0, 0.0);
		let mut disperse = vec2(0.0, 0.0);
		let mut cohere_count = 0.0;

		for other in flock.iter() {
			if other == self {
				continue;
			}

			let distance = other.position.distance(self.position);

			if distance < distances.align {
				align += other.velocity;
			}

			if distance < distances.cohere {
				cohere += other.position;
				cohere_count += 1.0;
			}

			if distance < distances.disperse {
				disperse += (self.position - other.position)
					/ self.position.distance_squared(other.position);
			}
		}

		if cohere_count > 0.0 {
			cohere = (cohere / cohere_count) - self.position;
		}

		self.acceleration = (self.normalize_steering_vector(seek) * weights.seek)
			+ (self.normalize_steering_vector(align) * weights.align)
			+ (self.normalize_steering_vector(cohere) * weights.cohere)
			+ (self.normalize_steering_vector(disperse) * weights.disperse);

		self.velocity = (self.velocity + self.acceleration).clamp_length_max(VELOCITY_LIMIT);
		self.position += self.velocity;
	}

	pub fn draw(&self, draw: &Draw) {
		draw.tri()
			.xy(self.position)
			.w_h(6.0, 4.0)
			.z_radians(self.velocity.angle())
			.color(self.color);

		#[cfg(debug_assertions)]
		draw.rect()
			.xy(self.position)
			.w_h(2.0, 2.0)
			.color(Lab::new(100.0, 0.0, 0.0));
	}

	fn wrap(&mut self, bounds: &Rect) {
		if self.position.x > bounds.right() {
			self.position.x = bounds.left();
		} else if self.position.x < bounds.left() {
			self.position.x = bounds.right();
		}

		if self.position.y > bounds.top() {
			self.position.y = bounds.bottom();
		} else if self.position.y < bounds.bottom() {
			self.position.y = bounds.top();
		}
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
