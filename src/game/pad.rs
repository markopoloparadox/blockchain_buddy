use crate::{
	game::{
		animate::{AnimateRange, Ease},
		buddy::{Side, Slot},
		Z_PAD,
	},
	AppState,
};
use bevy::prelude::*;
use std::time::Duration;

pub const PAD_SPACING: f32 = 180.0;
const SIDE_SPACING: f32 = 120.0;
const RIGHT_PAD_OUT: f32 = 1500.0;
const PAD_CENTER_OFFSET: f32 = ((Slot::MAX_PER_SIDE - 1) as f32 * PAD_SPACING) / 2.0;
const SHOP_PAD_OFFSET: f32 = -200.0;
const SHOP_PAD_OUT: f32 = -800.0;

#[derive(Bundle, Default)]
pub struct PadBundle {
	pub pad: Pad,
	pub slot: Slot,
	pub side: Side,
	pub transform: Transform,
	pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct Pad {
	right_animate_in: AnimateRange,
	right_animate_out: AnimateRange,
	left_animate_center: AnimateRange,
	left_animate_side: AnimateRange,
	shop_animate_in: AnimateRange,
	shop_animate_out: AnimateRange,
}

impl Default for Pad {
	fn default() -> Self {
		let mut value = Self {
			right_animate_out: AnimateRange::new(
				Duration::from_secs_f32(1.5),
				Ease::InOutCirc,
				SIDE_SPACING..RIGHT_PAD_OUT,
				false,
			),
			right_animate_in: AnimateRange::new(
				Duration::from_secs_f32(2.0),
				Ease::InOutCirc,
				RIGHT_PAD_OUT..SIDE_SPACING,
				false,
			),
			left_animate_center: AnimateRange::new(
				Duration::from_secs_f32(1.5),
				Ease::InOutCirc,
				-SIDE_SPACING..PAD_CENTER_OFFSET,
				false,
			),
			left_animate_side: AnimateRange::new(
				Duration::from_secs_f32(2.0),
				Ease::InOutCirc,
				PAD_CENTER_OFFSET..-SIDE_SPACING,
				false,
			),
			shop_animate_out: AnimateRange::new(
				Duration::from_secs_f32(1.5),
				Ease::InOutCirc,
				SHOP_PAD_OFFSET..SHOP_PAD_OUT,
				false,
			),
			shop_animate_in: AnimateRange::new(
				Duration::from_secs_f32(1.5),
				Ease::InOutCirc,
				SHOP_PAD_OUT..SHOP_PAD_OFFSET,
				false,
			),
		};
		value.right_animate_out.set_percent(1.0);
		value.left_animate_center.set_percent(1.0);
		value
	}
}

pub fn spawn_pads(commands: &mut Commands, asset_server: &AssetServer) {
	for i in 0..Slot::MAX_PER_SIDE {
		spawn_pad(commands, asset_server, Side::Left, Slot::new(i));
		spawn_pad(commands, asset_server, Side::Right, Slot::new(i));
	}
}

pub fn spawn_pad(commands: &mut Commands, asset_server: &AssetServer, side: Side, slot: Slot) {
	commands
		.spawn_bundle(PadBundle { side, slot, ..Default::default() })
		.with_children(|parent| {
			parent.spawn_bundle(SpriteBundle {
				texture: asset_server.load("pad.png"),
				transform: Transform::from_xyz(0., -60., Z_PAD),
				..Default::default()
			});
		});
}

pub fn position_pad(
	time: Res<Time>,
	state: Res<State<AppState>>,
	mut pads: Query<(&mut Pad, &mut Transform, &Side, &Slot)>,
) {
	for (mut pad, mut transform, side, slot) in pads.iter_mut() {
		let side_sign;
		let offset = match side {
			Side::Left => {
				side_sign = -1.0;
				if *state.current() == AppState::Battle {
					Vec2::new(pad.left_animate_side.tick(time.delta()), 0.0)
				} else {
					Vec2::new(pad.left_animate_center.tick(time.delta()), 0.0)
				}
			},
			Side::Right => {
				side_sign = 1.0;
				if *state.current() == AppState::Battle {
					Vec2::new(pad.right_animate_in.tick(time.delta()), 0.0)
				} else {
					Vec2::new(pad.right_animate_out.tick(time.delta()), 0.0)
				}
			},
			Side::Shop => {
				side_sign = -1.0;
				if *state.current() == AppState::Battle {
					Vec2::new(PAD_CENTER_OFFSET, pad.shop_animate_out.tick(time.delta()))
				} else {
					Vec2::new(PAD_CENTER_OFFSET, pad.shop_animate_in.tick(time.delta()))
				}
			},
		};

		let position = Vec2::new(slot.current as f32 * PAD_SPACING * side_sign, 0.0) + offset;
		*transform = Transform::from_translation(position.extend(0.0));
	}
}

pub fn pad_exit_battle(mut pads: Query<(&mut Pad, &Side)>) {
	for (mut pad, side) in pads.iter_mut() {
		match side {
			Side::Left => pad.left_animate_center.reset(),
			Side::Right => pad.right_animate_out.reset(),
			Side::Shop => pad.shop_animate_in.reset(),
		}
	}
}

pub fn pad_enter_battle(mut pads: Query<(&mut Pad, &Side)>) {
	for (mut pad, side) in pads.iter_mut() {
		match side {
			Side::Left => pad.left_animate_side.reset(),
			Side::Right => pad.right_animate_in.reset(),
			Side::Shop => pad.shop_animate_out.reset(),
		}
	}
}
