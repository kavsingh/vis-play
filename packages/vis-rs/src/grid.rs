use std::collections::HashMap;

use bevy::prelude::*;

use crate::boid::Boid;

pub struct SpatialGrid {
	cell_size: f32,
	cells: HashMap<(i32, i32), Vec<(Boid, Transform)>>,
}

impl SpatialGrid {
	pub fn new(cell_size: f32) -> Self {
		Self {
			cell_size,
			cells: HashMap::new(),
		}
	}

	pub fn clear(&mut self) {
		self.cells.clear();
	}

	pub fn insert(&mut self, boid: Boid, transform: Transform) {
		let cell = self.get_cell_index(transform.translation.truncate());
		self.cells.entry(cell).or_default().push((boid, transform));
	}

	pub fn get_neighbors(&self, position: Vec2, radius: f32) -> Vec<(&Boid, &Transform)> {
		let mut neighbors = Vec::new();
		let cell_radius = (radius / self.cell_size).ceil() as i32;
		let center_cell = self.get_cell_index(position);

		for dx in -cell_radius..=cell_radius {
			for dy in -cell_radius..=cell_radius {
				let cell = (center_cell.0 + dx, center_cell.1 + dy);
				if let Some(boids) = self.cells.get(&cell) {
					for (boid, transform) in boids {
						if position.distance(transform.translation.truncate()) <= radius {
							neighbors.push((boid, transform));
						}
					}
				}
			}
		}

		neighbors
	}

	fn get_cell_index(&self, position: Vec2) -> (i32, i32) {
		(
			(position.x / self.cell_size).floor() as i32,
			(position.y / self.cell_size).floor() as i32,
		)
	}
}
