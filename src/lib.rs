pub mod painter;
pub mod parser;
pub mod gui;
pub mod io_manager;

pub use painter::*;
pub use parser::*;
pub use gui::*;
pub use io_manager::*;

pub fn contains_in_vec(num: i32, vec: &[(i32, i32)]) -> bool {
    for (f, _) in vec {
        if *f == num {
            return true;
        }
    }
    return false;
}