use crate::unit::Unit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::hash::{Hash, Hasher};

// use serde_with::serde_as;

// #[derive(Debug, Clone)]
type UnitMap = HashMap<String, Unit>;

// #[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Units {
    // #[serde_as(as = "Vec<(_, _)>")]
    unit_list: UnitMap,
}

impl Units {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let v: UnitMap = HashMap::new();
        Self { unit_list: v }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, unit: Unit) {
        self.unit_list.insert(unit.id.clone(), unit);
    }

    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        let list = self.unit_list.len();
        list
    }

    #[allow(dead_code)]
    pub fn delete_unit(&mut self, id: String) {
        self.unit_list.remove(&id);
    }

    #[allow(dead_code)]
    pub fn get_units(&self) -> UnitMap {
        self.unit_list.clone()
    }

    /*
    pub fn get_last(&self) -> Unit {
        self.unit_list.
    */
}

/*
impl Hash for Units {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unit_list.hasher();
    }
}
*/
