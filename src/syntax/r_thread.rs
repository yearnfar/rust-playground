use std::sync::mpsc;
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
}
