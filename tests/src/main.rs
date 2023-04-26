use std::collections::HashMap;

fn main() {
    let mut integers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 6, 8, 1, 7, 8, 9, 10, 10];
    integers.sort();

    println!("Sorted: {:?}", integers);
    println!("Sorted: {}", integers.len());

    if integers.len() % 2 == 0 {
        let median = (integers[integers.len() / 2] + integers[integers.len() / 2 - 1]) / 2;
        println!("Median: {}", median);
    } else {
        let median = integers[integers.len() / 2];
        println!("Median: {}", median);
    }

    let mode = integers.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    println!("Mode: {:?}", mode);

    let mean = integers.iter().fold(0, |acc, x| acc + x) / integers.len() as i32;

    println!("Mean: {}", mean)
}
