use std::fs;
use serde::{Deserialize};
use serde_json::{Value};

const FILENAME: &str = "./resources/enemies.json";

#[derive(Debug, Deserialize)]
struct EnemyDrop {
	item_id: i32,
	rate: u8,
}

#[derive(Debug, Deserialize)]
struct Enemy {
	id: i32,
	name: String,
	hp: i32,
	strength: i32,
	defense: i32,
	magic: i32,
	#[serde(alias = "magicDefense")]
	magic_defense: i32,
	gil: i32,
	exp: i32,
	treasure: Vec<EnemyDrop>,
}

impl Enemy {
	fn make() -> Enemy {
		Enemy {
			id: 0,
			name: "",
			hp: 0,
			strength: 0,
			defense: 0,
			magic: 0,
			magic_defense: 0,
			gil: 0,
			exp: 0,
			treasure: Vec(),
		}
	}
}


fn main() {
	let contents = fs::read_to_string(FILENAME).expect("Unable to read file");
	let v: Vec<Enemy> = serde_json::from_str(&contents)
		.expect("Cannot deserialize data");

	println!("v: {:?}", v);
}
