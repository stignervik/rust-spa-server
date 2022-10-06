// use rust_spa_server::Potter;
use crate::unit::unit::Unit;
use rust_spa_server::unit;

use crate::units::Units;
use rust_spa_server::units;

#[test]
fn units_test() {
    let unit = Unit::new(
        String::from("1"),
        String::from("Unit1"),
        String::from("Unit"),
        String::from("Unit"),
    );
    println!("Unit id: {0}", unit.id());

    let mut units: Units = Units::new();
    units.push(unit);
    println!("Units len: {0}", units.length());

    units.length();
}
