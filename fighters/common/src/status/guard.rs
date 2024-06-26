use super::*;

mod guard_on;
mod guard;
mod guard_off;
mod guard_damage;

pub fn install() {
    guard_on::install();
    guard::install();
    guard_off::install();
    guard_damage::install();
}