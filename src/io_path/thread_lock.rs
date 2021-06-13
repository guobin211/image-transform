use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn generate_workout(intensity: u32, random_number: u32) {
    let expensive = |num| {
        println!("input {} , slowly ...", intensity);
        // 线程阻塞
        thread::sleep(Duration::from_secs(3));
        num + 1
    };

    if intensity < 25 {
        println!("now intensity < 25, {}", expensive(intensity));
        println!("next intensity < 25, {}", expensive(intensity));
    } else {
        if random_number == 3 {
            println!("random_number == 3");
        } else {
            println!("run for {}", expensive(intensity))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io_path::thread_lock::generate_workout;

    #[test]
    fn thread_test() {
        generate_workout(10, 3);
    }
}
