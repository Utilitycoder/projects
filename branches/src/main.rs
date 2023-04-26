fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let condition = false;

    let new_number = if condition { 1 } else { 0 };
    println!("new_number is: {new_number}");

    let mut num: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 6, 8, 1, 7, 8, 9, 10, 10];

    num.sort();

    let v = &num[9];

    println!("The value of v is: {}", v);
}
