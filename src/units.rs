use crate::unit::unit::Unit;

#[derive(Clone)]
pub struct Units {
    _unit_list: Vec<Unit>,
}

impl Units {
    pub fn new() -> Self {
        let v: Vec<Unit> = Vec::new();
        Self { _unit_list: v }
    }

    pub fn push(&mut self, unit: Unit) {
        self._unit_list.push(unit);
    }

    pub fn length(&self) -> usize {
        let list = self._unit_list.len();
        list
    }
}
