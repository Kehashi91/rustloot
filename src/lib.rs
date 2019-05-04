extern crate rand;
extern crate serde;

use rand::prelude::*;
use rand::distributions::WeightedIndex;
use serde::{Deserialize, Deserializer};
use serde::de::Error;

#[derive(Deserialize, Debug)]
pub struct Item {
	name: String,
	value: u16,
	weight: u16,
	#[serde(deserialize_with = "tranform_rarity")]
	rarity: u8,
	category: String,
}

fn tranform_rarity<'de, D>(deserializer: D) -> Result<u8, D::Error> 
where 
	D: Deserializer<'de> {
    let deserialized: Result<u8, D::Error> = Deserialize::deserialize(deserializer);
    match deserialized {
    	Ok(deserialized) => Ok(u8::max_value() - deserialized),
    	Err(_) => Err(D::Error::custom("Cannot convert rarity!")),
     }
}

#[derive(Debug)]
pub struct LootTable<'a> {
	biome: &'a str,
	base_multiplier: u16,
	items: Vec<&'a Item>,
	min_value: u16,
}

pub enum LootTableBuilders<'a> {
	AddCategory(String),
	AddItem(&'a str),
	AddAll,
	RemoveItem(String),
}

impl<'a>  LootTable<'a> {
	pub fn new(biome: &'a str, items: &'a Vec<Item>, base_multiplier: u16, builders: Vec<LootTableBuilders>) 
	-> LootTable<'a> {

		//let local_refs: Vec<&Item> = items.iter().collect();
		//et items_refs: Vec<&Item> = LootTable::buildershandler(items, &LootTableBuilders::AddAll); // filters to be added
		let items_refs: Vec<&Item> = LootTable::buildershandler(items, LootTableBuilders::AddItem("cmiks")); // filters to be added


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
			let mut temp_items: Vec<_> = self.items
				.clone()
				.into_iter()
				.filter(|item| item.value <= total_value as u16)
				.collect();

			let weights: Vec<u32> = temp_items.iter().map(|item| item.rarity as u32).collect();
			let dist = WeightedIndex::new(&weights).unwrap();

			println!("{:?}", temp_items);

			let index = dist.sample(random_thread);
			loot.push(temp_items[index]);
			total_value -= temp_items[index].value as f32;

			println!("{:?}", total_value);
		}

		loot
	}

	fn buildershandler<'b>(items: &'b Vec< Item>, builder_options: LootTableBuilders ) -> Vec<&'b Item> {
		match builder_options {
			LootTableBuilders::AddCategory(c) => items.iter().collect(),
			LootTableBuilders::AddItem(name) => items.iter().filter(|item| item.name == *name).collect(),
			LootTableBuilders::AddAll => items.iter().collect(),
			LootTableBuilders::RemoveItem(item) => items.iter().collect(),
		}
	}
}