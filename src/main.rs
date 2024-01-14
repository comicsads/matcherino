#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
struct Position {
	x: f64,
	y: f64,
}

impl Position {
	pub fn new(x: f64, y: f64) -> Self {
		Position { x, y }
	}
}

#[derive(Component)]
struct Draggable;

#[derive(Component)]
struct Jewel;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn((Jewel, Position::new(0.0, 0.0)));
}
