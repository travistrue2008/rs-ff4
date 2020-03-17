extern crate reqwest;

#[macro_use] extern crate scan_fmt;

use serde::{Serialize};
use serde_json;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

const URL: &'static str = "https://gamefaqs.gamespot.com/psp/615911-final-fantasy-iv-the-complete-collection/faqs/62210";

#[derive(Debug, Serialize)]
struct EnemyDrop {
	item_id: i32,
	rate: u8,
}

#[derive(Debug, Serialize)]
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
	fn make(id: i32) -> Enemy {
		Enemy {
			id,
			hp: 0,
			exp: 0,
			gil: 0,
			strength: 0,
			defense: 0,
			magic: 0,
			magic_defense: 0,
			name: String::new(),
			treasure: Vec::new(),
		}
	}
}

fn read_lines(contents: &str) -> Vec<String> {
	contents
		.split("\n")
		.filter(|line| -> bool {
			let trimmed = line.trim();

			trimmed.len() > 0 && trimmed.chars().next().unwrap() == '|'
		})
		.map(|line| line.replace("|", ""))
		.map(|line| String::from(line.trim()))
		.filter(|line| line.len() > 0)
		.collect()
}

fn process_stat(value: &str) -> i32 {
	match value.parse::<i32>() {
		Ok(num) => num,
		Err(_) => -1,
	}
}

fn process_lines(lines: &Vec<String>) -> Vec<Enemy> {
	let mut list: Vec<Enemy> = Vec::new();
	let mut item = Enemy::make(0);

	for line in lines {
		if let Ok((key, value)) = scan_fmt!(&line, "{}: {}", String, String) {
			match key.as_str() {
				"Number" => {
					let id = value.parse::<i32>().unwrap();

					if id != item.id {
						list.push(item);
						item = item::make(id);
					}
				},
				"Name" => (item.name = line.replace("Name: ", "")),
				"HP" => (item.hp = process_stat(&value)),
				"EXP" => (item.exp = process_stat(&value)),
				"Gil" => (item.gil = process_stat(&value)),
				"Strength" => (item.strength = process_stat(&value)),
				"Defense" => (item.defense = process_stat(&value)),
				"Magic" => (item.magic = process_stat(&value)),
				"Magic Def." => (item.magic_defense = process_stat(&value)),
				_ => {},
			}
		}
	}

	list.push(item);
	list.remove(0);
	list
}

fn write_file(items: &Vec<Enemy>) {
	let result = serde_json::to_string_pretty(&items).unwrap();
	let path = Path::new("./resources/enemies.json");
	let mut file = fs::File::create(&path).unwrap();

	file.write_all(result.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let res = reqwest::get(URL).await?.text().await?;
	let lines = read_lines(&res);
	let list = process_lines(&lines);

	write_file(&list);
	Ok(())
}
