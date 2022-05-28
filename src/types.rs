use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CharacterDetails {
	pub face: u16,
	pub health: u32,
	pub strength: u32,
	pub color: Color,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkMessage {
	Connect(String),
	Save(UserData),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkMessageResponse {
	Connect(UserData, Vec<CharacterDetails>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserData {
	pub coins: u32,
	pub trophies: (u32, u32),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GenericNetworkMessage {
	pub game_id: GameId,
	pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum GameId {
	BlockchainBuddy,
}
