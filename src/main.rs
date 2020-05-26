extern crate flatset;

use flatset::FlatSet;
use std::iter::FromIterator;

fn main() {
    let vec: Vec<usize> = (0..10).collect();
    let set = FlatSet::from_iter(vec);
    println!("set.len() -> {:?}", set.len());
    let needle = 4;
    // println!("{:?}", set.branchful_binary_search_contains(&needle) == true);
    println!("{:?}", set.branchless_binary_search_contains(&needle) == true);
    // println!("{:?}", set.branchful_level_order_search_contains(&needle) == true);
    // println!("{:?}", set.branchless_level_order_search_contains(&needle) == true);
}
