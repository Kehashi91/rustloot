extern crate rand;

use rand::prelude::*;

#[derive(Debug)]
pub struct Item {
	pub name: &'static str,
	pub value: u16,
	pub weight: u16,
	pub base_rarity: f32,
}

#[derive(Debug)]
pub struct LootTable<'a> {
	biome: &'a str,
	items: Vec<&'a Item>,
}

impl<'a>  LootTable<'a> {
	pub fn new(biome: &'a str, items: Vec<&'a Item>) -> LootTable<'a> {
		LootTable {
			biome: biome,
			items: items,
		}
	}
	pub fn roll(&self, score: u16, random_thread: & mut rand::prelude::ThreadRng) -> &Item {
		let temp_items: Vec<_> = self.items
			.clone()
			.into_iter()
			.filter(|item| item.value < score)
			.collect();
		temp_items[random_thread.gen_range(0, temp_items.len())]
	}
}