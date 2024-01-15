use bevy::prelude::*;
use matcherino::*;
// use rand::prelude::*;

#[cfg(debug_assertions)]
const WINDOW_HEIGHT: f32 = 2400.0 * 0.6;
#[cfg(debug_assertions)]
const WINDOW_WIDTH: f32 = 1080.0 * 0.6;

#[cfg(not(debug_assertions))]
const WINDOW_HEIGHT: f32 = 2400.0;
#[cfg(not(debug_assertions))]
const WINDOW_WIDTH: f32 = 1080.0;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Matcherino".to_string(),
				resolution: bevy::window::WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
				..default()
			}),
			..default()
		}))
		.add_plugins(OrbPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, start_drag)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
	/*
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
	*/
}
