mod normals;
mod smash;
// mod throws;
// mod aerials;
mod specials;
mod stance_movement;
mod stance_normals;
mod stance_specials;
mod escape;
// mod misc;

pub fn install() {
    normals::install();
    smash::install();
    // throws::install();
    // aerials::install();
    specials::install();
    stance_movement::install();
    stance_normals::install();
    stance_specials::install();
    escape::install();
    // misc::install();
}