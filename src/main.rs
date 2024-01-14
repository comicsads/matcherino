#![allow(dead_code)]
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Debug)]
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
struct Orb;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.add_systems(Update, update_sprites)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2dBundle::default());
	commands.spawn((
		Orb,
		Position::new(0.0, 0.0),
		SpriteBundle {
			texture: asset_server.load("sprites/orb.png"),
			transform: Transform {
				translation: Vec3::ZERO,
				scale: Vec3::new(0.1, 0.1, 0.0),
				..default()
			},
			..default()
		},
	));
}

fn move_orbs(time: Res<Time>, mut query: Query<&mut Position, With<Orb>>) {
	for mut pos in &mut query {
		pos.x += 50.0 * time.delta_seconds_f64();
	}
}

//TODO: This is probably a bad way of getting sprites to follow entities
fn update_sprites(mut query: Query<(&mut Transform, &Position)>) {
	for (mut tra, pos) in &mut query {
		tra.translation.x = pos.x as f32;
		tra.translation.y = pos.y as f32;
	}
}
