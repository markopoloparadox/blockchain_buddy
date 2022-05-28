#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod connect;
mod game;
mod menu;
mod network;
mod types;

use crate::{game::GamePlugin, menu::MenuPlugin};
use bevy::prelude::*;
use connect::ConnectPlugin;
use network::NetworkPlugin;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor { ..Default::default() })
		.insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
		.add_state(AppState::LoadNetwork)
		.add_plugins(DefaultPlugins)
		.add_plugin(NetworkPlugin)
		.add_plugin(ConnectPlugin)
		.add_plugin(MenuPlugin)
		.add_plugin(GamePlugin)
		.add_startup_system(setup)
		.run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
	LoadNetwork,
	Connect,
	Login,
	Menu,
	Startup,
	Shop,
	Battle,
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());
}
