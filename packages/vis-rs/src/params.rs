pub struct Distances {
	pub align: f32,
	pub cohere: f32,
	pub disperse: f32,
	pub max: f32,
}

impl Default for Distances {
	fn default() -> Self {
		let align = 25.0;
		let cohere = 50.0;
		let disperse = 25.0;

		Self {
			align,
			cohere,
			disperse,
			max: cohere.max(align.max(disperse)),
		}
	}
}

pub struct Weights {
	pub seek: f32,
	pub align: f32,
	pub cohere: f32,
	pub disperse: f32,
}

impl Default for Weights {
	fn default() -> Self {
		Self {
			seek: 1.0,
			align: 1.0,
			cohere: 1.0,
			disperse: 1.6,
		}
	}
}
