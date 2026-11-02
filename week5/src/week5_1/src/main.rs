use std::sync::Mutex;
use std::thread;
use std::sync::MutexGuard;
use std::time::Duration;
fn main() {
    let lock_a = Mutex::new(0);
    let lock_b = Mutex::new(0);
    let lock_c = Mutex::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let _a = lock_a.lock().unwrap();
            println!("thread 1: got lock A, waiting for B");

            let _b = lock_b.lock().unwrap();
            println!("thread 1: got lock B, waiting for C");

            let _c = lock_c.lock().unwrap();
            println!("thread 1: got both locks");
            thread::sleep(Duration::from_millis(100));
        });

        s.spawn(|| {
            let _a = lock_a.lock().unwrap();
            println!("thread 2: got lock A, waiting for B");

            let _b = lock_b.lock().unwrap();
            println!("thread 2: got lock B, waiting for C");

            let _c = lock_c.lock().unwrap();
            println!("thread 2: got both locks");
            thread::sleep(Duration::from_millis(100));
        });
    });

    println!("both threads finished");

fn lock_in_order<'a>(
    a: &'a Mutex<i32>,
    b: &'a Mutex<i32>,
) -> (MutexGuard<'a, i32>, MutexGuard<'a, i32>) {
    if (a as *const Mutex<i32> as usize) < (b as *const Mutex<i32> as usize) {
        let guard_a = a.lock().unwrap();
        let guard_b = b.lock().unwrap();
        (guard_a, guard_b)
    } else {
        let guard_b = b.lock().unwrap();
        let guard_a = a.lock().unwrap();
        (guard_a, guard_GNU nano 8.7.1                                                                                                                                                                                                                                                                                                                                                                                                                    main.rs *
use std::sync::Mutex;
use std::thread;
use std::sync::MutexGuard;
use std::time::Duration;
fn main() {
    let lock_a = Mutex::new(0);
    let lock_b = Mutex::new(0);
    let lock_c = Mutex::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let _a = lock_a.lock().unwrap();
            println!("thread 1: got lock A, waiting for B");

            let _b = lock_b.lock().unwrap();
            println!("thread 1: got lock B, waiting for C");

            let _c = lock_c.lock().unwrap();
            println!("thread 1: got both locks");
            thread::sleep(Duration::from_millis(100));
        });

        s.spawn(|| {
            let _a = lock_a.lock().unwrap();
            println!("thread 2: got lock A, waiting for B");

            let _b = lock_b.lock().unwrap();
            println!("thread 2: got lock B, waiting for C");

            let _c = lock_c.lock().unwrap();
            println!("thread 2: got both locks");
            thread::sleep(Duration::from_millis(100));
        });
    });

    println!("both threads finished");

fn lock_in_order<'a>(
    a: &'a Mutex<i32>,
    b: &'a Mutex<i32>,
) -> (MutexGuard<'a, i32>, MutexGuard<'a, i32>) {
    if (a as *const Mutex<i32> as usize) < (b as *const Mutex<i32> as usize) {
        let guard_a = a.lock().unwrap();
        let guard_b = b.lock().unwrap();
        (guard_a, guard_b)
    } else {
        let guard_b = b.lock().unwrap();
        let guard_a = a.lock().unwrap();
        (guard_a, guard_b)
    }
}
}
fn lock_in_order<'a>(
    a: &'a Mutex<i32>,
    b: &'a Mutex<i32>,
) -> (MutexGuard<'a, i32>, MutexGuard<'a, i32>) {
    if (a as *const Mutex<i32> as usize) < (b as *const Mutex<i32> as usize) {
        let guard_a = a.lock().unwrap();
        let guard_b = b.lock().unwrap();
        (guard_a, guard_b)
    } else {
        let guard_b = b.lock().unwrap();
        let guard_a = a.lock().unwrap();
        (guard_a, guard_b)
    }
}
