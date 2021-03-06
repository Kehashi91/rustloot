use rustloot::{Item, LootTable, LootTableBuilders};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let mut rng = rand::thread_rng();
    let items = read_items("items.json").unwrap();

    let builders = vec![LootTableBuilders::AddCategory("misc")];
    let builders2 = vec![LootTableBuilders::AddAll];

    let table = LootTable::new("test", &items, 2, builders);
    println!("{:?}", table.roll(15, &mut rng));
    println!("{:?}", table.roll(31, &mut rng));
    let table2 = LootTable::new("test2", &items, 3, builders2);
    println!("{:?}", table2.roll(15, &mut rng));
    println!("{:?}", table2.roll(31, &mut rng));
}

fn read_items<P: AsRef<Path>>(path: P) -> Result<Vec<Item>, Box<Error>> {
    // Read the json file with items
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let items = serde_json::from_reader(reader)?;

    Ok(items)
}
