use rustloot::LootTable;
use rustloot::Item;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
	let mut rng = rand::thread_rng();
	let items = read_items("items.json").unwrap();

	let table = LootTable::new("miasto", &items, 2);
	println!("{:?}", table.roll(15, & mut rng));
	println!("{:?}", table.roll(31, & mut rng));
	let table2 = LootTable::new("miast2o", &items, 3);
	println!("{:?}", table2.roll(15, & mut rng));
	println!("{:?}", table2.roll(31, & mut rng));
}

fn read_items<P: AsRef<Path>>(path: P) -> Result<Vec<Item>, Box<Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let items = serde_json::from_reader(reader)?;

	Ok(items)
}


