use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

struct ThreadPool<F> where F: Fn() -> () + Send + 'static {
    queue: Arc<Mutex<VecDeque<F>>>,
}

impl<F> ThreadPool<F> where F: Fn() -> () + Send + 'static {

    fn new(thread_count: u32) -> Self {
        let queue: Mutex<VecDeque<F>> = Mutex::new(VecDeque::new());
        let queue_ref = Arc::new(queue);

        for _ in 0..thread_count {
            let q = queue_ref.clone();
            std::thread::spawn(move || {
                loop {
                    let mut real_queue = q.lock().unwrap();
                    if !real_queue.is_empty() {
                        let work_item: F = real_queue.pop_front().unwrap();
                        drop(real_queue);
                        work_item();
                    }
                }
            });
        }
        return ThreadPool {
            queue: queue_ref
        };
    }

    fn spawn(self: &Self, f: F) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(f);
    }
}


pub fn main() {
    let pool = ThreadPool::new(1);
    for i in 0..4 {
        pool.spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(250 * i));
            println!("This is Task {}", i);
        });
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
}