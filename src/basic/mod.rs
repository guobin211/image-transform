use crate::basic::string_type::run;

mod data_type;
mod file_utils;
mod loop_fn;
mod oop_class;
mod string_type;

#[allow(dead_code)]
pub fn run_basic_mod() {
    let index = loop_fn::find_index(6);
    if index > 0 {
        println!("find index res is {}", index);
    }
    run()
}
