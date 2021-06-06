mod data_type;
mod file_utils;
mod loop_fn;
mod oop_class;

#[allow(unused_imports)]
use crate::oop_class::Person;
#[allow(unused_imports)]
use crate::oop_class::Sex;

fn main() {
    let index = loop_fn::find_index(6);
    if index > 0 {
        println!("find index res is {}", index);
    }
}
