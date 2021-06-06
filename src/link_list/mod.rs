mod array_op;
mod arrow_fn;
mod link_list_string;

pub fn run_link_list() {
    let mut list = link_list_string::LinkListNumber::new();
    list = list.prepend(11);
    println!("list len {}", list.len());
    println!("list {}", list.to_string());
    // while_let();
    // match_if();
    arrow_fn::run_arrow_fn();
    let fb = array_op::fibonacci();
    for i in fb.take(4) {
        println!("fb {}", i)
    }

    let mut array: Vec<&str> = vec!["A", "b", "c"];
    array.push("d");
    for (i, el) in array.iter().enumerate() {
        println!("array at {} is {}", i, el)
    }
}
