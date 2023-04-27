fn main() {
    println!("Hello, world!");

    another_function();


    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);
}

fn another_function() {
    println!("Another function.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_another_function() {
        assert_eq!(another_function(), ());
    }
}