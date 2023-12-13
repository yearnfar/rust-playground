use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn run() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(200));
    }

    handle.join().unwrap();
}

fn run_move() {
    let arr = vec![1, 2, 3];
    let arr2 = arr.clone();
    let handle = thread::spawn(move || {
        println!("the vec is {:?}", arr);
    });

    println!("the vec is {:?}", arr2);
    handle.join().unwrap();
}

fn run_channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let arr = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string(),
        ];

        for v in arr {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let arr = vec![
            "more".to_string(),
            "message".to_string(),
            "for".to_string(),
            "you".to_string(),
        ];

        for v in arr {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got: {}", received)
    }

    // let received = rx.recv().unwrap();
    // println!("got: {}", received)
}

fn run_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }
    println!("{:?}", m);
}

fn run_mutex2() {
    let cnt = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let cnt = Arc::clone(&cnt);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", cnt.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }

    #[test]
    fn test_run_move() {
        run_move();
    }

    #[test]
    fn test_run_channel() {
        run_channel();
    }

    #[test]
    fn test_run_mutex() {
        run_mutex();
    }

    #[test]
    fn test_run_mutex2() {
        run_mutex2();
    }
}
