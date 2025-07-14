use std::collections::HashMap;

use nannou::prelude::*;

use crate::boid::Boid;

pub struct SpatialGrid {
	cell_size: f32,
	cells: HashMap<(i32, i32), Vec<Boid>>,
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

	pub fn insert(&mut self, boid: Boid) {
		let cell = self.get_cell_index(boid.position());
		self.cells.entry(cell).or_default().push(boid);
	}

	pub fn get_neighbors(&self, position: Point2, radius: f32) -> Vec<&Boid> {
		let mut neighbors = Vec::new();
		let cell_radius = (radius / self.cell_size).ceil() as i32;
		let center_cell = self.get_cell_index(position);

		for dx in -cell_radius..=cell_radius {
			for dy in -cell_radius..=cell_radius {
				let cell = (center_cell.0 + dx, center_cell.1 + dy);
				if let Some(boids) = self.cells.get(&cell) {
					for boid in boids {
						if position.distance(boid.position()) <= radius {
							neighbors.push(boid);
						}
					}
				}
			}
		}

		neighbors
	}

	pub fn draw_debug(&self, draw: &Draw, bounds: &Rect) {
		// Draw grid lines
		let start_x = (bounds.left() / self.cell_size).floor() * self.cell_size;
		let end_x = (bounds.right() / self.cell_size).ceil() * self.cell_size;
		let start_y = (bounds.bottom() / self.cell_size).floor() * self.cell_size;
		let end_y = (bounds.top() / self.cell_size).ceil() * self.cell_size;

		// Vertical lines
		let mut x = start_x;
		while x <= end_x {
			draw.line()
				.start(pt2(x, bounds.bottom()))
				.end(pt2(x, bounds.top()))
				.color(rgba(0.2, 0.2, 0.2, 0.2))
				.weight(1.0);
			x += self.cell_size;
		}

		// Horizontal lines
		let mut y = start_y;
		while y <= end_y {
			draw.line()
				.start(pt2(bounds.left(), y))
				.end(pt2(bounds.right(), y))
				.color(rgba(0.2, 0.2, 0.2, 0.2))
				.weight(1.0);
			y += self.cell_size;
		}

		// Draw cell population counts
		for (&(cell_x, cell_y), boids) in &self.cells {
			let cell_center_x = cell_x as f32 * self.cell_size + self.cell_size / 2.0;
			let cell_center_y = cell_y as f32 * self.cell_size + self.cell_size / 2.0;

			// Draw count
			draw.text(&boids.len().to_string())
				.xy(pt2(cell_center_x, cell_center_y))
				.color(rgba(0.6, 0.6, 0.6, 0.8))
				.font_size(14);
		}
	}

	fn get_cell_index(&self, position: Point2) -> (i32, i32) {
		(
			(position.x / self.cell_size).floor() as i32,
			(position.y / self.cell_size).floor() as i32,
		)
	}
}
