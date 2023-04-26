fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };


    println!("the result is: {result}");

    labelled_loop();
    while_loop();
    while_for_array();
    forloop_array();
    println!("The current temperature is: {:?}", convert_temp(100.0, "c"));
    println!("The current temperature is: {:?}", convert_temp(100.0, "C"));
    let first = String::from("Abubakar");
    let full = test_moving(first.clone());
    println!("{full} which is previously {first}" );
}

fn labelled_loop() {
    let mut count = 0;
    'counting_up: loop { // labelled_loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LISTOFF");
}

fn while_for_array() {
    let a = [4, 6, 7, 19, 20];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

// Alternative to the above function which is faster as runtime code is lesser.
fn forloop_array() {
    let a = [4, 6, 7, 19, 20];

    for element in a {
        println!("The value is: {}", element);
    }
}

fn convert_temp(temp: f32, unit: &str) -> f32 {
    match unit {
        "F" | "f" => (temp - 32.0) * 5.0 / 9.0,
        "C" | "c" => temp * 9.0 / 5.0 + 32.0,
        _ => panic!("Invalid temperature unit"),
    }
}

fn test_moving(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
