use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;

use crate::AppState;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Network::new())
			.add_system_set(SystemSet::on_enter(AppState::LoadNetwork).with_system(enter_state))
			.add_system_set(SystemSet::on_exit(AppState::LoadNetwork).with_system(exit_state));
	}
}

pub struct Network {
	connection: Option<TcpStream>,
}

impl Network {
	pub fn new() -> Self {
		Self { connection: None }
	}

	pub fn send_message(&mut self, msg: &NetworkMessage) {
		let encoded: Vec<u8> = bincode::serialize(&msg).unwrap();
		self.send_message_internal(encoded.as_slice());
	}

	fn send_message_internal(&mut self, buf: &[u8]) {
		if let Some(conn) = &mut self.connection {
			conn.write(buf).unwrap();
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkMessageKind {
	Connect,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NetworkMessage {
	pub kind: NetworkMessageKind,
	pub data: Vec<u8>,
}

pub trait Networkable {
	fn to_network_message(&self) -> NetworkMessage;
}

pub fn enter_state(mut state: ResMut<State<AppState>>, mut res: ResMut<Network>) {
	println!("Enter LoadNetwork State");

	let stream = TcpStream::connect("127.0.0.1:8040");
	res.connection = stream.ok();

	state.set(AppState::Connect).unwrap();
}

pub fn exit_state() {
	println!("Exit LoadNetwork State");
}
