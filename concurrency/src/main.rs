use std::{thread, sync::mpsc, time::Duration};

fn main() {
    run_thread();
    msg_channel();
}

fn run_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
            println!("hi number {:?} from the spawned thread!", v);
            thread::sleep(Duration::from_millis(1));
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn msg_channel() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
        
    });

    for received in rx {
        println!("Got: {}", received);
    }
}