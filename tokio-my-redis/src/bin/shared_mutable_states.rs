use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedMap {
    inner: Arc<Mutex<SharedMapInner>>,
}

struct SharedMapInner {
    data: HashMap<i32, String>,
}

impl SharedMap {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SharedMapInner {
                data: HashMap::new(),
            }))
        }
    }

    pub fn insert(&self, key: i32, value: String) {
        let mut lock = self.inner.lock().unwrap();
        lock.data.insert(key, value);
    }

    pub fn get(&self, key: i32) -> Option<String> {
        let lock = self.inner.lock().unwrap();
        lock.data.get(&key).cloned()
    }
}



fn main() {
    let map = SharedMap::new();

    map.insert(10, "hello world".to_string());

    let map1 = map.clone();
    let map2 = map.clone();

    let thread1 = std::thread::spawn(move || {
        map1.insert(10, "foo bar".to_string());
    });

    let thread2 = std::thread::spawn(move || {
        let value = map2.get(10).unwrap();
        if value == "foo bar" {
            println!("Thread 1 was faster");
        } else {
            println!("Thread 2 was faster");
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}