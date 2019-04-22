extern crate rand;
extern crate serde;

use rand::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
	name: String,
	value: u16,
	weight: u16,
	is_rare: bool,
	category: String,
}

#[derive(Debug)]
pub struct LootTable<'a> {
	biome: &'a str,
	base_multiplier: u16,
	items: Vec<&'a Item>,
	min_value: u16,
}

impl<'a>  LootTable<'a> {
	pub fn new(biome: &'a str, items: &'a Vec<Item>, base_multiplier: u16) -> LootTable<'a> {

		let items_refs: Vec<&Item> = items.iter().collect(); // filters to be added

		let min_value: Vec<u16> = items_refs.iter().map(|x| x.value).collect();
		let min_value = min_value.into_iter().min().unwrap();

		LootTable {
			biome: biome,
			items: items_refs,
			base_multiplier: base_multiplier,
			min_value: min_value
		}
	}
	pub fn roll(&self, score: u16, random_thread: & mut rand::prelude::ThreadRng) -> Vec<&Item> {
		
		let mut total_value = score as f32 * self.base_multiplier as f32 * random_thread.gen_range(0.8, 1.2);
		let mut loot: Vec<&Item> = vec![];

		while total_value >= self.min_value as f32 {
			let temp_items: Vec<_> = self.items
				.clone()
				.into_iter()
				.filter(|item| item.value <= total_value as u16)
				.collect();
			println!("{:?}", temp_items);
			let item = temp_items[random_thread.gen_range(0, temp_items.len())];
			total_value -= item.value as f32;
			loot.push(item);
			println!("{:?}", total_value);
		}

		loot
	}
}