use std::mem;

const MAX_COUNT: u32 = 1000;

pub fn run_arrow_fn() {
    let color = "green";

    let log = || println!("`color` : {}", color);
    log();

    println!("color mem is {}", mem::size_of_val(color));

    let sum = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < MAX_COUNT)
        .filter(|&n| n % 2 == 1)
        .fold(0, |sum, i| sum + i);

    println!("max 1000 sum is {}", sum);
    println!("max 10 sum is {}", sum_odd_number(10));
}

fn sum_odd_number(up_to: u32) -> u32 {
    let mut res = 0;

    for i in 0..up_to {
        if i % 2 == 1 {
            res += i;
        }
    }

    res
}
