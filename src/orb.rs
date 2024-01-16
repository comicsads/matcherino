use crate::*;

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, add_orbs);
	}
}
pub fn add_orbs(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(OrbBundle::new(
		0.0,
		0.0,
		asset_server.load("sprites/orb.png"),
		OrbColor::Green,
	));
	commands.spawn(OrbBundle::new(
		75.0,
		75.0,
		asset_server.load("sprites/orb.png"),
		OrbColor::Blue,
	));
}

#[derive(Bundle)]
pub struct OrbBundle {
	orb: Orb,
	sprite: SpriteBundle,
	drag: Draggable,
	color: OrbColor,
}

#[derive(Component)]
pub enum OrbColor {
	Blue,
	Green,
}

impl OrbColor {
	pub fn to_bevy_color(&self) -> Color {
		match self {
			OrbColor::Blue => Color::Rgba {
				red: 1.0,
				green: 1.0,
				blue: 1.0,
				alpha: 1.0,
			},
			OrbColor::Green => Color::Rgba {
				red: 0.0,
				green: 1.0,
				blue: 0.0,
				alpha: 1.0,
			},
		}
	}
}

impl OrbBundle {
	pub fn new(pos_x: f32, pos_y: f32, texture: Handle<Image>, color: OrbColor) -> Self {
		OrbBundle {
			orb: Orb,
			sprite: SpriteBundle {
				transform: Transform {
					translation: Vec3::new(pos_x, pos_y, 0.0),
					scale: Vec3::new(0.5, 0.5, 100.0),
					..default()
				},
				texture,
				sprite: Sprite {
					color: color.to_bevy_color(),
					..default()
				},
				..default()
			},
			drag: Draggable,
			color,
		}
	}
}
