use std::collections::HashMap;

use bevy::prelude::*;

use crate::boids::Movement;

pub struct SpatialGrid {
	cell_size: f32,
	cells: HashMap<(i32, i32), Vec<(Entity, Movement)>>,
}

impl SpatialGrid {
	pub fn new(cell_size: f32) -> Self {
		Self {
			cell_size,
			cells: HashMap::new(),
		}
	}

	pub fn reset(&mut self, cell_size: Option<f32>) {
		self.cells.clear();

		if let Some(next_size) = cell_size {
			self.cell_size = next_size;
		}
	}

	pub fn insert(&mut self, entity: Entity, movement: Movement) {
		let cell_index = self.get_cell_index(&movement.position);

		self.cells
			.entry(cell_index)
			.or_default()
			.push((entity, movement));
	}

	pub fn get_neighbors(&self, position: &Vec2, radius: f32) -> Vec<(Entity, Movement, f32)> {
		let mut neighbors = Vec::new();
		let cell_radius = (radius / self.cell_size).ceil() as i32;
		let center_cell = self.get_cell_index(position);

		for dx in -cell_radius..=cell_radius {
			for dy in -cell_radius..=cell_radius {
				let cell = (center_cell.0 + dx, center_cell.1 + dy);
				if let Some(members) = self.cells.get(&cell) {
					for (e, m) in members {
						let distance = position.distance(m.position);
						if distance <= radius {
							neighbors.push((*e, *m, distance));
						}
					}
				}
			}
		}

		neighbors
	}

	fn get_cell_index(&self, position: &Vec2) -> (i32, i32) {
		(
			(position.x / self.cell_size).floor() as i32,
			(position.y / self.cell_size).floor() as i32,
		)
	}
}
