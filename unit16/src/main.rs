use std::{
    sync::{mpsc, Mutex, Arc},
    thread,
    time::Duration,
};

fn main() {
    // part1();
    // part2();
    // _part3();
    part4();
}
fn _part1() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(300));
        }
    });
    handle.join().unwrap();
    for i in 0..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(300));
    }
}

fn _part2() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });
    handle.join().unwrap();
}
fn _part3() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec!["Hi", "what", "is", "your", "name", "!"];
        for v in vals {
            tx.send(String::from(v)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec!["Hi-", "what-", "is-", "your-", "name-", "!-"];
        for v in vals {
            tx1.send(String::from(v)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });
    for recv in rx {
        println!("{}", recv);
    }
}

fn part4() {
    let m = Mutex::new(2);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:?}", m);

    let m_loc = Arc::new(Mutex::new(0));
    let mut handls = vec![];
    for _ in 0..10 {
        let m_loc = Arc::clone(&m_loc);
        let handle = thread::spawn(move || {
            let mut num = m_loc.lock().unwrap();
            *num += 1;
        });
        handls.push(handle);
    }
    for recv in handls {
        recv.join().unwrap();
    }
    println!("val:{}",m_loc.lock().unwrap());
}
