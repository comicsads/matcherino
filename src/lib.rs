use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position {
	pub x: f64,
	pub y: f64,
}

impl Position {
	pub fn new(x: f64, y: f64) -> Self {
		Position { x, y }
	}
}

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Orb;

// fn drag_draggables(mut query: Query< >)

pub fn move_orbs(time: Res<Time>, mut query: Query<&mut Position, With<Orb>>) {
	for mut pos in &mut query {
		pos.x += 50.0 * time.delta_seconds_f64();
	}
}
