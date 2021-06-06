use std::ops::Add;

/// 展示rust语言的基本数据类型
#[allow(dead_code)]
pub fn log_private_data_type() {
    let int8: i8 = -8;
    let int16: i16 = -16;
    let int32: i32 = -32;
    let int64: i64 = -64;
    let int128: i128 = -128;
    let size: isize = 4;
    println!("isize is {}", size);
    println!(
        "有符号整形 {0}, {1}, {2}, {3}, {4}",
        int8, int16, int32, int64, int128
    );

    let added = int8.add(8);
    println!("int8(-8) + 8 = {}", added);

    let uint8: u8 = 8;
    let uint16: u16 = 16;
    let uint32: u32 = 32;
    let uint64: u64 = 64;
    let uint128: u128 = 128;
    println!(
        "无符号整形 {0}, {1}, {2}, {3}, {4}",
        uint8, uint16, uint32, uint64, uint128
    );

    let f32: f32 = 32.0;
    let f64: f64 = 64.0;
    let bit = b'A';
    println!("浮点数和字节 {}, {}, {}", f32, f64, bit);

    let binary = 0b1111_0000;
    let oct = 0o77;
    let hexadecimal = 0xff;
    println!(
        "二进制，八进制，十六进制，{}, {}, {}",
        binary, oct, hexadecimal
    );

    if oct > binary {
        println!("oct > binary")
    } else {
        println!("oct <= binary")
    }

    let project_name: &'static str = "image-transform";
    let mut project_name_string: String = String::from("image-transform");

    assert_eq!(project_name, project_name_string);

    let list = ["Jason", "Peter", "Jerry"];
    for el in list.iter() {
        println!("i am {}", el);
        project_name_string.push_str(" ");
        project_name_string.push_str(el);
    }

    let res = list.concat();
    println!("list concat {}", res);
    println!("project name {}", project_name_string);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("元组数据 {:?}", tup);
}
