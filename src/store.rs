use std::sync::{Arc, RwLock};
use crate::units::Units;
// use serde::{Deserialize, Serialize};

#[derive(Clone)]

pub struct Store {
  units: Arc<RwLock<Units>>
}

impl Store {
  pub fn new() -> Self {
    Self {units: Arc::new(RwLock::new(Units::new()))}
  }
  pub fn units(&self) -> Arc<RwLock<Units>> {
    self.units.clone()
  }
}