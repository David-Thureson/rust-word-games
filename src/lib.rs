#![feature(drain_filter)]

pub use itertools::Itertools;
pub use util::*;
pub use util::format::fc;

pub mod jumble;
pub mod word_list;
pub mod word_search;

pub fn slice_str_to_strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|x| x.to_string()).collect()
}

pub fn vec_str_to_strings(list: &Vec<&str>) -> Vec<String> {
    list.iter().map(|x| x.to_string()).collect()
}

