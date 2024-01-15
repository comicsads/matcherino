use bevy::prelude::*;
use matcherino::*;

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, update_sprites)
			.add_systems(Startup, add_orbs);
	}
}

fn add_orbs(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(OrbBundle::new(
		0.0,
		0.0,
		asset_server.load("sprites/orb.png"),
	));
}

#[derive(Bundle)]
pub struct OrbBundle {
	orb: Orb,
	sprite: SpriteBundle,
	drag: Draggable,
}

impl OrbBundle {
	pub fn new(pos_x: f32, pos_y: f32, texture: Handle<Image>) -> Self {
		OrbBundle {
			orb: Orb,
			sprite: SpriteBundle {
				transform: Transform {
					translation: Vec3::new(pos_x, pos_y, 0.0),
					scale: Vec3::new(0.1, 0.1, 100.0),
					..default()
				},
				texture,
				..default()
			},
			drag: Draggable,
		}
	}
}

//TODO: This is probably a bad way of getting sprites to follow entities
fn update_sprites(mut query: Query<(&mut Transform, &Position)>) {
	for (mut tra, pos) in &mut query {
		tra.translation.x = pos.x as f32;
		tra.translation.y = pos.y as f32;
	}
}
