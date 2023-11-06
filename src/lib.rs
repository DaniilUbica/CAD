pub mod painter;
pub mod parser;
pub mod gui;
pub mod io_manager;
pub mod processor;

use std::collections::HashMap;

pub use painter::*;
pub use parser::*;
pub use gui::*;
pub use io_manager::*;
pub use processor::*;

use std::ops::Add;

pub fn contains_in_vec<T: Add<Output = T> + std::cmp::PartialEq>(num: T, vec: &[(T, T)]) -> bool {
    for (f, _) in vec {
        if *f == num {
            return true;
        }
    }
    return false;
}

pub fn vec_to_map<T: Add<Output = T> + std::cmp::Eq + std::hash::Hash + Copy>(vec: &[(T, T)]) -> HashMap<T, T> {
    let mut m = HashMap::new();

    for (k, v) in vec {
        m.entry(*k).or_insert(*v);
    }

    m
}