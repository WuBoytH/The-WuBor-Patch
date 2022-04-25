mod specials;
mod stance_wait;
mod stance_movement;
mod stance_attacks;
mod stance_specials;
pub mod helper;

pub fn install() {
    specials::install();
    stance_wait::install();
    stance_movement::install();
    stance_attacks::install();
    stance_specials::install();
}