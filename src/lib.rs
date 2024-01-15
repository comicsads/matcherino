use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position {
	x: f64,
	y: f64,
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

//TODO: This is probably a bad way of getting sprites to follow entities
pub fn update_sprites(mut query: Query<(&mut Transform, &Position)>) {
	for (mut tra, pos) in &mut query {
		tra.translation.x = pos.x as f32;
		tra.translation.y = pos.y as f32;
	}
}
