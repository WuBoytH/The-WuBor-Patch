mod specials;
mod wait_noble;
mod movement_noble;
mod attacks_noble;
mod specials_noble;
pub mod helper;

pub fn install() {
    specials::install();
    wait_noble::install();
    movement_noble::install();
    attacks_noble::install();
    specials_noble::install();
}