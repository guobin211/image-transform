use std::convert::TryInto;

#[allow(dead_code)]
pub(crate) fn print_loop_fn(c: char) {
    let s = ['R', 'U', 'N', 'O', 'O', 'B', 'o', '0', 'i', 'I', 'L', 'l'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == c {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);
}

#[allow(dead_code)]
pub(crate) fn find_max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut index = 0;
    while index < array.len() {
        if array[index] > array[max_index] {
            max_index = index;
        }
        index += 1;
    }
    array[max_index]
}

#[allow(dead_code)]
pub(crate) fn find_index(t: u32) -> isize {
    let data: [u32; 7] = [9, 10, 11, 12, 6, 7, 8];
    let index = data.iter().position(|&el| el == t);
    return match index {
        Some(v) => {
            println!("find index is {}", v);
            v.try_into().unwrap()
        }
        _ => {
            println!("not found {}", t);
            -1
        }
    };
}

#[allow(dead_code)]
pub(crate) fn print_for_loop() {
    let list = [1, 2, 3, 4, 5];
    for x in list.iter() {
        println!("current is {}", x);
    }
}
