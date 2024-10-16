use core::hint::black_box;
use skirt::mutex::Mutex;

fn main() {
    let mutex = Mutex::new(361);
    black_box(mutex.lock());
}
