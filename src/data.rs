// Use once_cell for creating a global variable e.g. our DATA.
use once_cell::sync::Lazy;

// Use Mutex for thread-safe access to a variable e.g. our DATA.
use std::sync::Mutex;

// Use HashMap for storing data as key-value pairs e.g. our DATA.
use std::collections::HashMap;

// Use the Book struct.
use crate::book::Book;

use crate::unit::unit::Unit;

// Create a data store as a global variable with `Lazy` and `Mutex`.
//
// This demo implementation uses a `HashMap` for ease and speed.
// The map key is a primary key for lookup; the map value is a Book.
//
// To access data, create a thread, spawn it, and acquire the lock:
//
// ```
// async fn example() {
//     thread::spawn(move || {
//         let data = DATA.lock().unwrap();
//         …
// }).join().unwrap()
// ```
pub static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            1,
            Book {
                id: 1,
                title: "Antigone".into(),
                author: "Sophocles".into(),
            },
        ),
        (
            2,
            Book {
                id: 2,
                title: "Beloved".into(),
                author: "Toni Morrison".into(),
            },
        ),
        (
            3,
            Book {
                id: 3,
                title: "Candide".into(),
                author: "Voltaire".into(),
            },
        ),
    ]))
});

// Create a unit data store as a global variable with `Lazy` and `Mutex`.
//
// This demo implementation uses a `HashMap` for ease and speed.
// The map key is a primary key for lookup; the map value is a Book.
//
// To access data, create a thread, spawn it, and acquire the lock:
//
// ```
// async fn example() {
//     thread::spawn(move || {
//         let data = UNIT_STORE.lock().unwrap();
//         …
// }).join().unwrap()
// ```

pub static UNIT_STORE: Lazy<Mutex<HashMap<u32, Unit>>> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            1,
            Unit {
                id: 1,
                name: "Unit 1".into(),
                unit_class: "Unit".into(),
                unit_func: "Unit".into(),
            },
        ),
        (
            2,
            Unit {
                id: 2,
                name: "Unit 2".into(),
                unit_class: "Unit".into(),
                unit_func: "Unit".into(),
            },
        ),
        (
            3,
            Unit {
                id: 3,
                name: "Unit 3".into(),
                unit_class: "Unit".into(),
                unit_func: "Unit".into(),
            },
        ),
    ]))
});
