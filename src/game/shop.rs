use crate::{
    game::{
        buddy::{spawn_buddy, Buddy, BuddyBundle, BuddyColor, BuddyFace, Side, Slot},
        counters::{set_coin_text, Coins},
        pad::{position_pad, spawn_pad},
        ui::UiRoot,
    },
    menu::{HOVERED_BUTTON, NORMAL_BUTTON},
    AppState,
};
use bevy::{
    math::{const_vec2, Vec3Swizzles},
    prelude::*,
    render::camera::CameraPlugin,
};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Startup).with_system(spawn_shop_base))
            .add_system_set(SystemSet::on_enter(AppState::Shop).with_system(enter_shop))
            .add_system_set(
                SystemSet::on_update(AppState::Shop)
                    .with_system(set_coin_text)
                    .with_system(position_pad)
                    .with_system(buy_buddy)
                    .with_system(battle_button),
            )
            .add_system_set(SystemSet::on_exit(AppState::Shop).with_system(exit_shop));
    }
}

const SHOP_BUDDY_SLOTS: usize = 3;

#[derive(Component)]
pub struct ShopPad;

pub fn spawn_shop_base(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..SHOP_BUDDY_SLOTS {
        spawn_pad(&mut commands, &asset_server, Side::Shop, Slot::new(i));
    }
}

pub struct ShopState {
    battle_button: Entity,
}

pub fn enter_shop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_root: Query<Entity, With<UiRoot>>,
    buddies: Query<(Entity, &Side), With<Buddy>>,
) {
    let ui_root = ui_root.single();
    let battle_button = spawn_battle_button(&mut commands, &asset_server, ui_root);
    commands.insert_resource(ShopState { battle_button });

    // clean up old shop entities
    for (entity, side) in buddies.iter() {
        if *side == Side::Shop {
            commands.entity(entity).despawn_recursive();
        }
    }

    for i in 0..SHOP_BUDDY_SLOTS {
        let buddy = BuddyBundle {
            color: BuddyColor::random(),
            slot: Slot::new(i),
            face: BuddyFace::random(),
            side: Side::Shop,
            ..Default::default()
        };
        let buddy_id = spawn_buddy(&mut commands, &asset_server, buddy);
        commands.entity(buddy_id).insert(Price(2));
    }
}

pub fn exit_shop(mut commands: Commands, shop_state: Res<ShopState>) {
    commands
        .entity(shop_state.battle_button)
        .despawn_recursive();
}

#[derive(Component)]
pub struct BattleButton;

fn spawn_battle_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    ui_root: Entity,
) -> Entity {
    let mut battle_button = None;
    commands.entity(ui_root).with_children(|parent| {
        battle_button = Some(
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(BattleButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Battle",
                            TextStyle {
                                font: asset_server.load("font/CaveatBrush-Regular.ttf"),
                                font_size: 60.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .id(),
        );
    });

    battle_button.unwrap()
}

pub fn battle_button(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<BattleButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::Battle).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

const BUDDY_EXTENTS: Vec2 = const_vec2!([65.0, 65.0]);

fn buy_buddy(
    mut coins: ResMut<Coins>,
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut buddies: Query<(&Transform, &mut Slot, &mut Side, &Price), With<Buddy>>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, global_transform) = cameras
        .iter()
        .find(|(camera, _)| camera.name.as_deref() == Some(CameraPlugin::CAMERA_2D))
        .unwrap();
    let cursor_screen = if let Some(cursor) = window.cursor_position() {
        cursor
    } else {
        return;
    };

    let cursor_world = screen_to_world(
        Vec2::new(window.width(), window.height()),
        cursor_screen,
        camera,
        global_transform,
    );
    if mouse_button.just_pressed(MouseButton::Left) {
        let occupied_slots = buddies
            .iter()
            .filter_map(|(_, slot, side, _)| {
                if *side == Side::Left {
                    Some(slot.0)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (transform, mut slot, mut side, price) in buddies.iter_mut() {
            if *side != Side::Shop {
                continue;
            }
            let pos = transform.translation;
            let min = pos.xy() - BUDDY_EXTENTS;
            let max = pos.xy() + BUDDY_EXTENTS;
            if cursor_world.x < max.x
                && cursor_world.x > min.x
                && cursor_world.y < max.y
                && cursor_world.y > min.y
            {
                let open_slot = (0..3).find(|i| !occupied_slots.contains(i));
                if let Some(open_slot) = open_slot {
                    *side = Side::Left;
                    *slot = Slot(open_slot);
                    coins.0 -= price.0;
                }
                break;
            }
        }
    }
}

fn screen_to_world(
    window_size: Vec2,
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
}

#[derive(Component)]
pub struct Price(usize);
