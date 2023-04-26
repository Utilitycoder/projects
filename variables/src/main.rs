use rand::Rng;
use std::io;

fn main() {
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    let tup: (i32, f64, u32) = (500, 6.4, 100);
    let (x, y, z) = tup;
    let five_hundred = x;
    let six_point_four = y;
    let hundred = z;

    println!("The value of the tup is: {five_hundred}, {six_point_four} and {hundred}");

    let mut array: [i32; 5] = [0; 5];

    for i in 0..5 {
        array[i] = rand::thread_rng().gen_range(1..=100)
    }

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Failed to parse line");

    let element = array[index];

    let array_value = array;
    println!("The array at index 2 is: {}", array[2]);
    println!("The array value is: {:?}", array_value);
    println!("The value at the index entered is: {:?}", element);
}
