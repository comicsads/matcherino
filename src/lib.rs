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
pub struct Dragging;

#[derive(Component)]
pub struct Orb;

pub fn start_drag(
	mut query: Query<&mut Transform, With<Draggable>>,
	window_query: Query<&Window>,
	buttons: Res<Input<MouseButton>>,
) {
	let _ = query;
	let _ = window_query;
	let _ = buttons;
	if buttons.pressed(MouseButton::Left) {
		for mut sprite_pos in &mut query {
			let single = &window_query.single();
			if let Some(m_pos) = single.cursor_position() {
				let x = m_pos.x - single.width() / 2.0;
				let y = m_pos.y - single.height() / 2.0;
				let y = y * -1.0;
				let real_pos = Vec3::new(x, y, 100.0);

				let collide = bevy::sprite::collide_aabb::collide(
					sprite_pos.translation,
					Vec2::new(30.0, 30.0),
					real_pos,
					Vec2::new(30.0, 30.0),
				);

				match collide {
					Some(_) => {
						sprite_pos.translation.x = real_pos.x;
						sprite_pos.translation.y = real_pos.y;
					}
					None => (),
				}
			}
		}
	}
}

// pub fn drag()

pub fn move_orbs(time: Res<Time>, mut query: Query<&mut Position, With<Orb>>) {
	for mut pos in &mut query {
		pos.x += 50.0 * time.delta_seconds_f64();
	}
}
