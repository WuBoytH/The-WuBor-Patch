mod smash_attacks;
mod throws;
mod aerials;
mod specials;
mod misc;

pub fn install() {
    smash_attacks::install();
    throws::install();
    aerials::install();
    specials::install();
    misc::install();
}