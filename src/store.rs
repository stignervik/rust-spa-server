/*
use std::sync::{Mutex};
use std::collections::HashMap;
use crate::unit::unit::Unit;
use once_cell::sync::Lazy;
*/
// pub static WORD_STORE: Arc::new<Mutex::new<Vec<String>>>;

// pub static WORD_STORE: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

/*
pub static STORE_UNIT: Lazy<Mutex<HashMap<u32, Unit>>> = Lazy::new(Mutex::new(HashMap::from([
  (
      1,
      Unit {
          id: 1,
          name: "Unit 1".into(),
          unit_class: "Unit".into(),
          unit_func: "Unit".into(),
      },
  ),])));
  */

/*
  pub static STORE_UNIT: Lazy<Mutex<HashMap<u32, Unit>>> = Lazy::new(|| {
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
*/
