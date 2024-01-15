use bevy::prelude::*;

mod orb;
pub use orb::OrbPlugin;

const GEM_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Dragging;

#[derive(Component)]
pub struct Orb;

pub fn start_drag(
	query: Query<(&Transform, Entity), With<Draggable>>,
	window_query: Query<&Window>,
	buttons: Res<Input<MouseButton>>,
	mut commands: Commands,
) {
	if buttons.just_pressed(MouseButton::Left) {
		for (sprite_pos, e_id) in query.iter() {
			let single = window_query.single();
			if let Some(m_pos) = single.cursor_position() {
				let mouse_pos_adjusted = adjust_mouse_pos(m_pos, single);

				let collide = bevy::sprite::collide_aabb::collide(
					sprite_pos.translation,
					Vec2::new(GEM_SIZE, GEM_SIZE),
					mouse_pos_adjusted,
					Vec2::new(GEM_SIZE, GEM_SIZE),
				);

				match collide {
					Some(_) => {
						commands.entity(e_id).insert(Dragging);
					}
					None => (),
				}
			}
		}
	}
}

pub fn time_to_drag(
	mut query: Query<(&mut Transform, Entity), With<Dragging>>,
	window_query: Query<&Window>,
	buttons: Res<Input<MouseButton>>,
	mut commands: Commands,
) {
	//TODO: This function is being run when supposedly no entities have Dragging component
	for (mut sprite_pos, e_id) in &mut query {
		let main_window = &window_query.single();
		if let Some(m_pos) = main_window.cursor_position() {
			let mouse_pos_adjusted = adjust_mouse_pos(m_pos, main_window);
			sprite_pos.translation.x = mouse_pos_adjusted.x;
			sprite_pos.translation.y = mouse_pos_adjusted.y;
		}
		if !buttons.pressed(MouseButton::Left) {
			commands.entity(e_id).remove::<Dragging>();
		}
	}
}

fn adjust_mouse_pos(m_pos: Vec2, single: &Window) -> Vec3 {
	let x = m_pos.x - single.width() / 2.0;
	let y = m_pos.y - single.height() / 2.0;
	let y = y * -1.0;

	let mouse_pos_adjusted = Vec3::new(x, y, 100.0);
	mouse_pos_adjusted
}
