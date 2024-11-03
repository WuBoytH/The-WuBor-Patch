use super::*;

mod guard_on;
mod guard;
mod guard_off;
mod guard_damage;

mod shield_break_fly;

pub fn install() {
    guard_on::install();
    guard::install();
    guard_off::install();
    guard_damage::install();

    shield_break_fly::install();
}