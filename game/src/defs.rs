use std::fs;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub enum ArmorKind {
	Shield,
	Helm,
	Armor,
	Robe,
	Ring,
	Gauntlets,
}

#[derive(Debug, Deserialize)]
pub enum WeaponKind {
	Sword,
	DarkSword,
	HolySword,
	Hammer,
	Katana,
	Knife,
	Axe,
	Bow,
	Arrow,
	Boomerang,
	Claw,
	Harp,
	Rod,
	Staff,
	Spear,
	Whip,
}

#[derive(Debug, Deserialize)]
pub enum Handedness {
	Left,
	Right,
	Both,
}

#[derive(Debug, Deserialize)]
pub struct Item {
	id: i32,
	price: i32,
	name: String,
	description: String,
}

#[derive(Debug, Deserialize)]
pub struct Armor {
	id: i32,
	price: i32,
	defense: i32,
	evasion: i32,
	#[serde(alias = "magicDefense")]
	magic_defense: i32,
	#[serde(alias = "magicEvasion")]
	magic_evastion: i32,
	name: String,
	#[serde(alias = "type")]
	kind: ArmorKind,
	modifiers: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Weapon {
	id: i32,
  price: i32,
	attack: i32,
	#[serde(alias = "hitRate")]
	hit_rate: i32,
	name: String,
	#[serde(alias = "type")]
  kind: WeaponKind,
  modifiers: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EnemyDrop {
	#[serde(alias = "itemId")]
	item_id: i32,
	quantity: i32,
	rate: u8,
}

#[derive(Debug, Deserialize)]
pub struct Enemy {
	id: i32,
	hp: i32,
	strength: i32,
	defense: i32,
	magic: i32,
	#[serde(alias = "magicDefense")]
	magic_defense: i32,
	gil: i32,
	exp: i32,
	name: String,
	treasure: Vec<EnemyDrop>,
}

impl Enemy {
	pub fn id(&self) -> i32 {
		self.id
	}

	pub fn hp(&self) -> i32 {
		self.hp
	}

	pub fn strength(&self) -> i32 {
		self.strength
	}

	pub fn defense(&self) -> i32 {
		self.defense
	}

	pub fn magic(&self) -> i32 {
		self.magic
	}

	pub fn magic_defense(&self) -> i32 {
		self.magic_defense
	}

	pub fn gil(&self) -> i32 {
		self.gil
	}

	pub fn exp(&self) -> i32 {
		self.exp
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn treasure(&self) -> &Vec<EnemyDrop> {
		&self.treasure
	}
}

#[derive(Debug, Deserialize)]
pub struct Command {
	name: String,
	description: String,
}

#[derive(Debug, Deserialize)]
pub struct PartyMember {
	id: i32,
	name: String,
	class: String,
	handedness: Handedness,
	#[serde(alias = "weaponTypes")]
	weapon_kinds: Vec<WeaponKind>,
	commands: Vec<Command>,
}

pub struct Definitions {
	items: Vec<Item>,
	armor: Vec<Armor>,
	weapons: Vec<Weapon>,
	enemies: Vec<Enemy>,
	members: Vec<PartyMember>,
}

impl Definitions {
	pub fn items(&self) -> &Vec<Item> {
		&self.items
	}

	pub fn armor(&self) -> &Vec<Armor> {
		&self.armor
	}

	pub fn weapons(&self) -> &Vec<Weapon> {
		&self.weapons
	}

	pub fn enemies(&self) -> &Vec<Enemy> {
		&self.enemies
	}

	pub fn members(&self) -> &Vec<PartyMember> {
		&self.members
	}
}

pub fn load() -> Definitions {
	let item_contents = fs::read_to_string("./assets/definitions/items.json").unwrap();
	let armor_contents = fs::read_to_string("./assets/definitions/armor.json").unwrap();
	let weapon_contents = fs::read_to_string("./assets/definitions/weapons.json").unwrap();
	let enemy_contents = fs::read_to_string("./assets/definitions/enemies.json").unwrap();
	let member_contents = fs::read_to_string("./assets/definitions/party.json").unwrap();

	Definitions {
		items: serde_json::from_str(&item_contents).unwrap(),
		armor: serde_json::from_str(&armor_contents).unwrap(),
		weapons: serde_json::from_str(&weapon_contents).unwrap(),
		enemies: serde_json::from_str(&enemy_contents).unwrap(),
		members: serde_json::from_str(&member_contents).unwrap(),
	}
}
