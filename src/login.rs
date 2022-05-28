/* use crate::AppState;
use bevy::{prelude::*, render::camera::Camera2d};

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Entities::new())
			.add_system_set(SystemSet::on_enter(AppState::Login).with_system(login_menu))
			.add_system_set(SystemSet::on_exit(AppState::Login).with_system(login_exit))
			.add_system(print_char_event_system)
			.add_system(button_system);
	}
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct EnterSeedLabel;

#[derive(Component)]
struct EnterSeedTextBox;

#[derive(Component)]
struct LoginButton;

pub struct Entities {
	label: Option<Entity>,
	text_box: Option<Entity>,
	button: Option<Entity>,
}

impl Entities {
	pub fn new() -> Self {
		Self { label: None, text_box: None, button: None }
	}
}

const NORMAL_BUTTON: UiColor = UiColor(Color::rgb(0.15, 0.15, 0.15));

pub fn login_exit(mut commands: Commands, asset_server: Res<AssetServer>, entities: Res<Entities>) {
	commands.entity(entities.label.unwrap()).despawn_recursive();
	commands.entity(entities.text_box.unwrap()).despawn_recursive();
	commands.entity(entities.button.unwrap()).despawn_recursive();
}

pub fn login_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut entities: ResMut<Entities>,
) {
	// Create Label that displays "Enter Seed"
	let label_id = commands
		.spawn_bundle(TextBundle {
			text: Text::with_section(
				"Enter seed: ",
				TextStyle {
					font: asset_server.load("font/IndieFlower-Regular.ttf"),
					font_size: 100.0,
					color: Color::WHITE,
				},
				Default::default(),
			),
			style: Style {
				position_type: PositionType::Absolute,
				position: Rect { top: Val::Px(5.0), right: Val::Px(400.0), ..Default::default() },
				..Default::default()
			},
			..Default::default()
		})
		.insert(EnterSeedLabel)
		.id();

	// Create TextBox
	let text_box_id = commands
		.spawn_bundle(TextBundle {
			text: Text::with_section(
				"",
				TextStyle {
					font: asset_server.load("font/IndieFlower-Regular.ttf"),
					font_size: 100.0,
					color: Color::WHITE,
				},
				Default::default(),
			),
			style: Style {
				position_type: PositionType::Absolute,
				position: Rect { top: Val::Px(80.0), right: Val::Px(400.0), ..Default::default() },
				..Default::default()
			},
			..Default::default()
		})
		.insert(EnterSeedTextBox)
		.id();

	let button_id = commands
		.spawn_bundle(ButtonBundle {
			style: Style {
				size: Size::new(Val::Px(150.0), Val::Px(65.0)),
				// center button
				margin: Rect::all(Val::Auto),
				// horizontally center child text
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..default()
			},
			..Default::default()
		})
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"Login",
					TextStyle {
						font: asset_server.load("font/IndieFlower-Regular.ttf"),
						font_size: 100.0,
						color: Color::BLACK,
					},
					Default::default(),
				),
				..Default::default()
			});
		})
		.insert(LoginButton)
		.id();

	entities.button = Some(button_id);
	entities.label = Some(label_id);
	entities.text_box = Some(text_box_id);
}

fn button_system(
	mut interaction_query: Query<
		(&Interaction, &mut UiColor, &Children),
		(Changed<Interaction>, With<Button>),
	>,
	mut text_query: Query<&mut Text>,
	mut app_state: ResMut<State<AppState>>,
) {
	for (interaction, mut color, children) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				app_state.set(AppState::Menu).unwrap();
				return;
			},
			_ => {},
		}
	}
}

/// This system prints out all char events as they come in
fn print_char_event_system(
	mut char_input_events: EventReader<ReceivedCharacter>,
	mut query: Query<&mut Text, With<EnterSeedTextBox>>,
) {
	if query.is_empty() {
		return;
	}

	let mut text = query.single_mut();

	for event in char_input_events.iter() {
		// Return button
		if event.char == '\u{8}' {
			text.sections[0].value.pop();
		} else if event.char == '\u{16}' {
		} else {
			text.sections[0].value.push(event.char);
		}

		info!("{:?}", text.sections[0].value);
	}
}

/* pub fn exit_blockchain_login(mut commands: Commands) {
		let mut left_slots = Vec::new();
	for (entity, slot, side) in buddies.iter_mut() {
		if *side == Side::Left {
			left_slots.push((entity, slot.base));
		}
	}
	left_slots.sort_by_key(|(_, slot)| *slot);
	for (new_slot, (entity, _slot)) in left_slots.iter().enumerate() {
		let mut slot = buddies.get_component_mut::<Slot>(*entity).unwrap();
		*slot = Slot::new(new_slot);
	}
	commands.entity(shop_state.battle_button).despawn_recursive();
	commands.entity(shop_state.trash).despawn_recursive();
}
 */
 */
