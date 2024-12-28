use nannou::color::Lab;
use nannou::prelude::*;
use uuid::Uuid;

const FORCE_LIMIT: f32 = 0.2;
const VELOCITY_LIMIT: f32 = 4.0;

#[derive(Clone)]
pub struct Boid {
	id: String,
	position: Point2,
	velocity: Vec2,
	acceleration: Vec2,
	color: Lab,
}

pub struct Weights {
	pub seek: f32,
	pub align: f32,
	pub cohere: f32,
	pub separate: f32,
}

impl Default for Weights {
	fn default() -> Self {
		Self {
			seek: 1.0,
			align: 1.0,
			cohere: 1.0,
			separate: 1.6,
		}
	}
}

impl Boid {
	pub fn create(bounds: &Rect) -> Boid {
		Boid {
			id: Uuid::new_v4().to_string(),
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
		attractors: &[Point2],
		weights: &Weights,
		bounds: &Rect,
	) {
		self.wrap(bounds);

		self.acceleration = self.seek(attractors) * weights.seek
			+ self.align(flock) * weights.align
			+ self.cohere(flock) * weights.cohere
			+ self.separate(flock) * weights.separate;
		self.position += self.velocity;
		self.velocity = (self.velocity + self.acceleration).clamp_length_max(VELOCITY_LIMIT);
	}

	pub fn draw(&self, draw: &Draw) {
		draw.ellipse()
			.x_y(self.position.x, self.position.y)
			.w_h(3.0, 3.0)
			.color(self.color);
	}

	fn seek(&self, attractors: &[Point2]) -> Vec2 {
		if attractors.is_empty() {
			return vec2(0.0, 0.0);
		}

		let total = attractors.iter().fold(vec2(0.0, 0.0), |acc, attractor| {
			acc + (*attractor - self.position) / (attractor.distance(self.position))
		});

		self.normalize_steering_vector(total / attractors.len() as f32)
	}

	fn align(&self, flock: &[Boid]) -> Vec2 {
		let group = self.local_group(flock, 25.0);

		if group.is_empty() {
			return vec2(0.0, 0.0);
		}

		let total = group
			.iter()
			.fold(vec2(0.0, 0.0), |acc, boid| acc + boid.velocity);

		self.normalize_steering_vector(total / group.len() as f32)
	}

	fn cohere(&self, flock: &[Boid]) -> Vec2 {
		let group = self.local_group(flock, 50.0);

		if group.is_empty() {
			return vec2(0.0, 0.0);
		}

		let total = group
			.iter()
			.fold(vec2(0.0, 0.0), |acc, boid| acc + boid.position);

		self.normalize_steering_vector((total / group.len() as f32) - self.position)
	}

	fn separate(&self, flock: &[Boid]) -> Vec2 {
		let group = self.local_group(flock, 25.0);

		if group.is_empty() {
			return vec2(0.0, 0.0);
		}

		let total = group.iter().fold(vec2(0.0, 0.0), |acc, boid| {
			acc + (self.position - boid.position) / (self.position.distance_squared(boid.position))
		});

		self.normalize_steering_vector(total / group.len() as f32)
	}

	fn local_group<'a>(&self, flock: &'a [Boid], radius: f32) -> Vec<&'a Boid> {
		flock
			.iter()
			.filter(|boid| *boid != self && boid.position.distance(self.position) < radius)
			.collect()
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
		let target = v.normalize() * VELOCITY_LIMIT;

		(target - self.velocity).clamp_length_max(FORCE_LIMIT)
	}
}

impl PartialEq for Boid {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
