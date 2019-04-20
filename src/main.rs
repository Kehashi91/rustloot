use rustloot::LootTable;
use rustloot::Item;

fn main() {
	let mut rng = rand::thread_rng();

    let cmiks = Item { 
    name: "cmiks",
	value: 2,
	weight: 0,
	base_rarity: 0.25
	};
	let shell = Item { 
    name: "łuska",
	value: 1,
	weight: 0,
	base_rarity: 0.4
	};
	let pistol = Item { 
    name: "m1911",
	value: 30,
	weight: 2,
	base_rarity: 0.1
	};

	let table = LootTable::new("miasto", vec![&cmiks, &shell, &pistol]);
	println!("{:?}", table.roll(31, & mut rng));
	println!("{:?}", table.roll(31, & mut rng));
}
