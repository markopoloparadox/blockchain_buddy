use bevy::prelude::*;
use std::io::stdin;

use crate::{
	network::{Network, NetworkMessage, NetworkMessageKind, Networkable},
	AppState,
};

pub struct ConnectPlugin;

impl Plugin for ConnectPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(ConnectInformation::new())
			.add_system_set(SystemSet::on_enter(AppState::Connect).with_system(enter_state))
			.add_system_set(SystemSet::on_exit(AppState::Connect).with_system(exit_state));
	}
}

pub struct ConnectInformation {
	account_seed: Option<String>,
}

impl Networkable for ConnectInformation {
	fn to_network_message(&self) -> NetworkMessage {
		if let Some(seed) = self.account_seed.clone() {
			NetworkMessage { kind: NetworkMessageKind::Connect, data: seed.as_bytes().to_vec() }
		} else {
			panic!("aaa")
		}
	}
}

impl ConnectInformation {
	pub fn new() -> Self {
		Self { account_seed: None }
	}
}

pub fn enter_state(
	mut state: ResMut<State<AppState>>,
	mut conn_info: ResMut<ConnectInformation>,
	mut net: ResMut<Network>,
) {
	println!("Enter Connect State");

	let mut account_seed = String::new();
	println!("Enter your Account Seed: ");
	stdin().read_line(&mut account_seed).ok().expect("qed");
	account_seed.pop();

	conn_info.account_seed = Some(account_seed);
	let msg = conn_info.to_network_message();
	net.send_message(&msg);

	state.set(AppState::Menu).unwrap();
}

pub fn exit_state() {
	println!("Exit Connect State");
}
