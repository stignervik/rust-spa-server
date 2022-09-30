// use rust_spa_server::Potter;
use rust_spa_server::unit;
use crate::unit::unit::Unit;

use rust_spa_server::units;
use crate::units::Units;

#[test]
fn units_test() {
  let unit = Unit::new(String::from("1"), String::from("Unit1"), String::from("Unit"), String::from("Unit"));
  print!("Unit id: {0}", unit.id());

  let mut units: Units = Units::new(Vec::new());
  units.push(unit);
  print!("Units len: {0}", units.length());
}
