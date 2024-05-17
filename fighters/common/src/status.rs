use super::*;

pub mod movement;
mod guard;
pub mod escape;
mod damage;
pub mod attack;
mod catch;

mod fall;

mod passive;
pub mod cliff;
mod appeal;
mod rebirth;
mod sub;
mod sub_transitions;
mod sub_fighter;

mod sub_set_status;

pub fn install() {
    movement::install();
    guard::install();
    escape::install();
    damage::install();
    attack::install();
    catch::install();

    fall::install();

    passive::install();
    cliff::install();
    appeal::install();
    rebirth::install();
    sub::install();
    sub_transitions::install();
    sub_fighter::install();

    sub_set_status::install();
}