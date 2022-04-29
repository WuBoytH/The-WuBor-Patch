mod normals;
mod smash;
// mod throws;
// mod aerials;
mod specials;
mod stance_movement;
mod stance_normals;
// mod misc;

pub fn install() {
    normals::install();
    smash::install();
    // throws::install();
    // aerials::install();
    specials::install();
    stance_movement::install();
    stance_normals::install();
    // misc::install();
}