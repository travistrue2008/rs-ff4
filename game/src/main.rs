mod defs;

fn main() {
	let definitions: defs::Definitions = defs::load();

	for enemy in definitions.enemies() {
		println!("enemies: {:?}", enemy);
	}
}
