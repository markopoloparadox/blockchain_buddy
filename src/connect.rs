use bevy::prelude::*;

use crate::game::counters::{Coins, Trophies};
use crate::network::Network;
use crate::types::NetworkMessage;
use crate::types::NetworkMessageResponse;
use crate::AppState;

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

impl ConnectInformation {
	pub fn new() -> Self {
		Self { account_seed: None }
	}
}

pub fn enter_state(
	mut state: ResMut<State<AppState>>,
	mut _conn_info: ResMut<ConnectInformation>,
	mut net: ResMut<Network>,
	mut coins: ResMut<Coins>,
	mut trophies: ResMut<Trophies>,
) {
	/* 	let mut account_seed = String::new();
	println!("Enter your Account Seed: ");
	stdin().read_line(&mut account_seed).ok().expect("qed");
	account_seed.pop();

	conn_info.account_seed = Some(account_seed); */

	let msg = NetworkMessage::Connect("aaa".to_string());
	net.send_message(&msg);

	let response = net.read_message().unwrap();
	match response {
		NetworkMessageResponse::Connect(user_data, _) => {
			*coins = Coins(user_data.coins as usize);
			trophies.won = user_data.trophies.0 as usize;
			trophies.rounds = user_data.trophies.1 as usize;
		},
	}

	state.set(AppState::Menu).unwrap();
}

pub fn exit_state() {}
