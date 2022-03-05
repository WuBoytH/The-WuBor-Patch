mod smash_attacks;
mod aerials;
mod specials;
mod misc;

pub fn install() {
    smash_attacks::install();
    aerials::install();
    specials::install();
    misc::install();
}