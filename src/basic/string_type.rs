use rand::prelude::*;
use std::ops::Add;

pub fn run() {
    let mut s = create_string(1);
    println!("s is {}", s);
    s = s.add("B");
    println!("now s is {}", s);

    let mut list: Vec<char> = Vec::new();
    list.push('A');
    list.push('B');
    println!("now s is {:?}", list);
}

fn create_string(len: u32) -> String {
    let mut rg = thread_rng();
    let r: u32 = rg.gen_range(1..=10);
    println!("random is {}", r);
    String::from("A").repeat(len as usize)
}
