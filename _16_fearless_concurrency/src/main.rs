use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration, vec};

fn main() {
    // thread
    block_until_spawned_finish();
    move_ownership_to_thread();
    // channel
    message_passing_through_channel();
    multiple_messages();
    multiple_messages_from_multiple_transmitters();
    // mutex
    try_mutex();
    sharing_mutex();
}

fn block_until_spawned_finish() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn move_ownership_to_thread() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}

fn message_passing_through_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = "hi".to_owned();
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn multiple_messages() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            "hi".to_owned(),
            "from".to_owned(),
            "the".to_owned(),
            "thread".to_owned()
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got:{received}");
    }
}

fn multiple_messages_from_multiple_transmitters() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            "hi".to_owned(),
            "from".to_owned(),
            "the".to_owned(),
            "thread".to_owned()
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            "more".to_owned(),
            "messages".to_owned(),
            "for".to_owned(),
            "you".to_owned()
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got:{received}");
    }
}

fn try_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {m:?}");
}

fn sharing_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}