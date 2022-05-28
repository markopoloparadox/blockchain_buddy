use bevy::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::game::counters::{Coins, Trophies};
use crate::types::{
	GameId, GenericNetworkMessage, NetworkMessage, NetworkMessageResponse, UserData,
};
use crate::AppState;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Network::new())
			.insert_resource(ShouldSaveGame(false))
			.add_system_set(SystemSet::on_enter(AppState::LoadNetwork).with_system(enter_state))
			.add_system_set(SystemSet::on_exit(AppState::LoadNetwork).with_system(exit_state))
			.add_system(enter_save_data_state);
	}
}

pub struct Network {
	connection: Option<TcpStream>,
}

pub struct ShouldSaveGame(pub bool);

impl Network {
	pub fn new() -> Self {
		Self { connection: None }
	}

	pub fn send_message(&mut self, msg: &NetworkMessage) {
		let encoded_data: Vec<u8> = bincode::serialize(&msg).unwrap();

		let msg = GenericNetworkMessage { game_id: GameId::BlockchainBuddy, data: encoded_data };
		let encoded: Vec<u8> = bincode::serialize(&msg).unwrap();

		self.send_message_internal(encoded.as_slice());
	}

	pub fn read_message(&mut self) -> Option<NetworkMessageResponse> {
		if let Some(conn) = &mut self.connection {
			let mut buf = [0; 2048];
			let count = conn.read(&mut buf).unwrap();
			let buf = buf[0..count].to_vec();

			let decoded: NetworkMessageResponse = bincode::deserialize(&buf).unwrap();
			return Some(decoded);
		}

		None
	}

	fn send_message_internal(&mut self, buf: &[u8]) {
		if let Some(conn) = &mut self.connection {
			conn.write(buf).unwrap();
		}
	}
}

pub fn enter_state(mut state: ResMut<State<AppState>>, mut res: ResMut<Network>) {
	let stream = TcpStream::connect("127.0.0.1:8040");
	res.connection = stream.ok();

	state.set(AppState::Connect).unwrap();
}

pub fn exit_state() {}

pub fn enter_save_data_state(
	mut save_game: ResMut<ShouldSaveGame>,
	coins: Res<Coins>,
	trophies: Res<Trophies>,
	mut network: ResMut<Network>,
) {
	if save_game.0 == true {
		let msg = NetworkMessage::Save(UserData {
			coins: coins.0 as u32,
			trophies: (trophies.won as u32, trophies.rounds as u32),
		});
		network.send_message(&msg);
	}

	save_game.0 = false;
}
