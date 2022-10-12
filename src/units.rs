use crate::unit::unit::Unit;
use std::collections::HashMap;

type UnitMap = HashMap<u32, Unit>;

#[derive(Clone)]
pub struct Units {
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
        self.unit_list.insert(unit.id, unit);
    }

    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        let list = self.unit_list.len();
        list
    }

    #[allow(dead_code)]
    pub fn get_units(&self) -> UnitMap {
        self.unit_list.clone()
    }
}
