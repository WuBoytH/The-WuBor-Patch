mod normals;
mod smash_attacks;
mod throws;
mod aerials;
mod specials;
mod escape;
mod misc;

pub fn install() {
    normals::install();
    smash_attacks::install();
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
    misc::install();
}