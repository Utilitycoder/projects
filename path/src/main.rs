use path::{my_house::my_kitchen::cook, my_office::{my_desk, my_chair}};

fn main() {
    println!("Hello, world!");
    cook();
    my_desk::work();
    let mut chair = my_chair::sit();
    chair.message = String::from("sitting on a chair!");
    chair.name = String::from("Modular");
    println!("The name of the chair is {} while \"{}\" is the message writing on it.", chair.name, chair.message);
}
