mod smash_attacks;
mod throws;
mod normals;
mod aerials;
mod specials;
mod escape;
// mod misc;

pub fn install() {
    smash_attacks::install();
    throws::install();
    normals::install();
    aerials::install();
    specials::install();
    escape::install();
    // misc::install();
}