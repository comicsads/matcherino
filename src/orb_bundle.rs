use bevy::prelude::*;
use matcherino::*;

#[derive(Bundle)]
struct OrbBundle {
	orb: Orb,
	sprite: SpriteBundle,
	pos: Position,
}

impl OrbBundle {
	fn new(filepath: String, pos_x: f64, pos_y: f64) -> Self {
		OrbBundle {
			orb: Orb,
			sprite: todo!(),
			pos: Position::new(pos_x, pos_y),
		}
	}
}
