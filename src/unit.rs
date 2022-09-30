pub mod unit {
  pub struct Unit {
    pub id: String,
    pub name: String,
    pub unit_class: String,
    pub unit_func: String,
  }

  impl Unit {
    pub fn new(id: String, name: String, unit_class: String, unit_func: String) -> Self {
      Self { id, name, unit_class, unit_func }
  }
    pub fn id(&self) -> &str {
      &self.id
    }
  }
}



