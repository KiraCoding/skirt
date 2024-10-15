use std::{sync::Arc, thread};
use skirt::mutex::Mutex;

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let _guard = mutex.lock();

    dbg!(&mutex);

    let mut handles = vec![];
    for _ in 0..4 {
        let lock = Arc::clone(&mutex);

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut guard = lock.lock();
                *guard += 1;
            }
        });
        handles.push(handle);
    }

    drop(_guard);

    for handle in handles {
        handle.join().unwrap();
    }

    let guard = mutex.lock();

    assert_eq!(*guard, 400);
}
