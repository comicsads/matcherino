use bevy::{input::mouse::MouseButtonInput, prelude::*};

mod orb;
pub use orb::OrbPlugin;

const GEM_SIZE: f32 = 75.0;
const GRID_SIZE: f32 = GEM_SIZE;

pub struct Point {
	x: f32,
	y: f32,
}

#[derive(Event)]
pub struct DragMoved {
	pub started: Point,
	pub current: Point,
}

/// If an entity can be dragged it will have this Component
#[derive(Component)]
pub struct Draggable;

/// If entity is currently being dragged.
/// Stores the coords where dragging begun.
#[derive(Component)]
pub struct Dragging {
	p: Point,
}

#[derive(Component)]
pub struct Orb;

/// Apply dragging to any entity mouse clicks that has Draggable
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

					if let Some(_) = collide {
						commands.entity(e_id).insert(Dragging {
							p: Point {
								x: sprite_pos.translation.x,
								y: sprite_pos.translation.y,
							},
						});
						break;
					}
				}
			}
		}
	}
}

/// Dragging orbs functionality
pub fn time_to_drag(
	mut query: Query<(&mut Transform, Entity, &Dragging)>,
	window_query: Query<&Window>,
	buttons: Res<Input<MouseButton>>,
	mut commands: Commands,
	mut send_event_orb_dragged: EventWriter<DragMoved>,
) {
	//TODO: This function is being run when supposedly no entities have Dragging component
	for (mut sprite_pos, e_id, drag) in &mut query {
		let main_window = &window_query.single();
		if let Some(m_pos) = main_window.cursor_position() {
			let mouse_pos_adjusted = adjust_mouse_pos(m_pos, main_window);

			let dist_dragged_x = drag.p.x - mouse_pos_adjusted.x;
			let dist_dragged_y = drag.p.y - mouse_pos_adjusted.y;

			let dist_dragged_x = dist_dragged_x.abs();
			let dist_dragged_y = dist_dragged_y.abs();

			if dist_dragged_x > dist_dragged_y {
				let clamped_mouse_pos = mouse_pos_adjusted
					.x
					.clamp(drag.p.x - GRID_SIZE, drag.p.x + GRID_SIZE);
				sprite_pos.translation.x = (clamped_mouse_pos / GRID_SIZE).round() * GRID_SIZE;
				sprite_pos.translation.y = drag.p.y;
			} else {
				let clamped_mouse_pos = mouse_pos_adjusted
					.y
					.clamp(drag.p.y - GRID_SIZE, drag.p.y + GRID_SIZE);
				sprite_pos.translation.x = drag.p.x;
				sprite_pos.translation.y = (clamped_mouse_pos / GRID_SIZE).round() * GRID_SIZE;
			}
			if drag.p.x != sprite_pos.translation.x || drag.p.y != sprite_pos.translation.y {
				send_event_orb_dragged.send(DragMoved {
					current: Point {
						x: sprite_pos.translation.x,
						y: sprite_pos.translation.y,
					},
					started: Point {
						x: drag.p.x,
						y: drag.p.y,
					},
				});
			}
		}
		if !buttons.pressed(MouseButton::Left) {
			commands.entity(e_id).remove::<Dragging>();
		}
	}
}

fn am_being_dragged(
	mut events: EventReader<DragMoved>,
	mut query: Query<&mut Transform, (With<Orb>, Without<Dragging>)>,
) {
	for my_event in events.read() {
		for mut tra in &mut query {
			if my_event.current.x == tra.translation.x || my_event.current.y == tra.translation.y {
				tra.translation.x = my_event.started.x;
				tra.translation.y = my_event.started.y;
			}
		}
	}
}

/// Converts raw mouse position to coords in window
fn adjust_mouse_pos(m_pos: Vec2, single: &Window) -> Vec3 {
	let x = m_pos.x - single.width() / 2.0;
	let y = m_pos.y - single.height() / 2.0;
	let y = y * -1.0;

	Vec3::new(x, y, 100.0)
}
