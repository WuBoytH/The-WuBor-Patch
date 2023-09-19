mod jump_aerial;
mod guard_on;
mod guard;
mod guard_damage;
pub mod helper;

pub fn install() {
    jump_aerial::install();
    guard_on::install();
    guard::install();
    guard_damage::install();
}