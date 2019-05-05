//! # RustLoot
//!
//! Simple rust cli program to generate random sample of items from a given loottable.
//! Given a json file with items, and by specifying a list of options, a loot table is build.
//! Then the roll method will give us a weighted sample from given loottable.

extern crate itertools;
extern crate rand;
extern crate serde;

use itertools::Itertools;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Item {
    // Represenation of items in lootable. It is important to distinguish beetween
    // value and rarity, both are important for rolls but for diffrent purpose
    name: String,
    value: u16,
    weight: u16,
    #[serde(deserialize_with = "tranform_rarity")]
    rarity: u8,
    category: String,
}

fn tranform_rarity<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    // We invert the rarity to make bigger rarity to mean that an item is more rare
    // We have to do that beacouse WeightedIndex inteprets bigger weight (that is
    // derived from rarity) as more common.
    let deserialized: Result<u8, D::Error> = Deserialize::deserialize(deserializer);
    match deserialized {
        Ok(deserialized) => Ok(u8::max_value() - deserialized),
        Err(_) => Err(D::Error::custom("Cannot convert rarity!")),
    }
}

#[derive(Debug)]
pub struct LootTable<'a> {
    // Container for items that can be rolled, as well as rules that can influence
    // rolls (for now only base multiplier)
    name: &'a str,
    base_multiplier: u16,
    items: Vec<&'a Item>,
    min_value: u16,
}

#[derive(Debug)]
pub enum LootTableBuilders<'a> {
    // These are 'commands' that can be used to build a loottable.
    // todo: build lootables based on YAML or similiar files.
    AddCategory(&'a str),
    AddItem(&'a str),
    AddAll,
    RemoveItem(&'a str),
}

impl<'a> LootTable<'a> {
    pub fn new(
        name: &'a str,
        items: &'a Vec<Item>,
        base_multiplier: u16,
        builders: Vec<LootTableBuilders>,
    ) -> LootTable<'a> {
        // Build a loottable based on vec of LootTableBuilders

        let mut items_refs: Vec<&Item> = vec![];

        for builder in builders.into_iter() {
            LootTable::buildershandler(items, &mut items_refs, builder);
        }

        let items_refs = LootTable::deduplicate(items_refs); // Ensure no duplicates

        let min_value: Vec<u16> = items_refs.iter().map(|x| x.value).collect();
        let min_value = min_value.into_iter().min().unwrap(); // get value of least valuable item

        LootTable {
            name: name,
            items: items_refs,
            base_multiplier: base_multiplier,
            min_value: min_value,
        }
    }

    pub fn roll(&self, score: u16, random_thread: &mut rand::prelude::ThreadRng) -> Vec<&Item> {
        // Get a loot from a given roll. Score is a roll value from a player, which determines the total value
        // of items in a roll. What items will be in the loot is determined by items rarity.

        let mut loot_value =
            score as f32 * self.base_multiplier as f32 * random_thread.gen_range(0.8, 1.2);
        let mut loot: Vec<&Item> = vec![];

        while loot_value >= self.min_value as f32 {
            let temp_items: Vec<_> = self
                .items
                .clone()
                .into_iter()
                .filter(|item| item.value <= loot_value as u16)
                .collect();

            let weights: Vec<u32> = temp_items.iter().map(|item| item.rarity as u32).collect();
            let dist = WeightedIndex::new(&weights).unwrap();

            println!("{:?}", temp_items);

            let index = dist.sample(random_thread);
            loot.push(temp_items[index]);
            loot_value -= temp_items[index].value as f32;
        }

        loot
    }

    fn buildershandler<'b>(
        items: &'b Vec<Item>,
        current_items: &'a mut Vec<&'b Item>,
        builder_options: LootTableBuilders,
    ) {
        match builder_options {
            LootTableBuilders::AddCategory(category) => {
                items
                    .iter()
                    .filter(|item| item.category == *category)
                    .for_each(|item| current_items.push(item));
            }
            LootTableBuilders::AddItem(name) => {
                items
                    .iter()
                    .filter(|item| item.name == *name)
                    .for_each(|item| current_items.push(item));
            }
            LootTableBuilders::AddAll => {
                items.iter().for_each(|item| current_items.push(item));
            }
            LootTableBuilders::RemoveItem(_) => unimplemented!(),
        }
    }

    fn deduplicate<'b>(current_items: Vec<&'b Item>) -> Vec<&Item> {
        current_items
            .into_iter()
            .unique_by(|item| &item.name)
            .collect()
    }
}
