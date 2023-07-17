mod guard_on;
mod guard;
mod guard_damage;
pub mod helper;

pub fn install() {
    guard_on::install();
    guard::install();
    guard_damage::install();
}