use bevy::{input::mouse::MouseButtonInput, prelude::*};

mod orb;
pub use orb::OrbPlugin;

const GEM_SIZE: f32 = 75.0;
const GRID_SIZE: f32 = GEM_SIZE;

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Dragging {
	start_x: f32,
	start_y: f32,
}

#[derive(Component)]
pub struct Orb;

pub fn start_drag(
	query: Query<(&Transform, Entity), With<Draggable>>,
	window_query: Query<&Window>,
	mut commands: Commands,
	mut mouse_event_reader: EventReader<MouseButtonInput>,
) {
	for mouse_event in mouse_event_reader.read() {
		if mouse_event.button == MouseButton::Left && mouse_event.state.is_pressed() {
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
							commands.entity(e_id).insert(Dragging {
								start_x: sprite_pos.translation.x,
								start_y: sprite_pos.translation.y,
							});
							break;
						}
						None => (),
					}
				}
			}
		}
	}
}

pub fn time_to_drag(
	mut query: Query<(&mut Transform, Entity, &Dragging)>,
	window_query: Query<&Window>,
	buttons: Res<Input<MouseButton>>,
	mut commands: Commands,
) {
	//TODO: This function is being run when supposedly no entities have Dragging component
	for (mut sprite_pos, e_id, drag) in &mut query {
		let main_window = &window_query.single();
		if let Some(m_pos) = main_window.cursor_position() {
			let mouse_pos_adjusted = adjust_mouse_pos(m_pos, main_window);

			let dist_dragged_x = drag.start_x - mouse_pos_adjusted.x;
			let dist_dragged_y = drag.start_y - mouse_pos_adjusted.y;

			let dist_dragged_x = dist_dragged_x.abs();
			let dist_dragged_y = dist_dragged_y.abs();

			if dist_dragged_x > dist_dragged_y {
				sprite_pos.translation.x = (mouse_pos_adjusted.x / GRID_SIZE).round() * GRID_SIZE;
				sprite_pos.translation.y = drag.start_y;
			} else {
				sprite_pos.translation.x = drag.start_x;
				sprite_pos.translation.y = (mouse_pos_adjusted.y / GRID_SIZE).round() * GRID_SIZE;
			}
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
