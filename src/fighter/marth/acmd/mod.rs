// mod ground_normals;
// mod smash_attacks;
// mod throws;
// mod aerials;
mod specials;
mod stance_normals;
// mod misc;

pub fn install() {
    // ground_normals::install();
    // smash_attacks::install();
    // throws::install();
    // aerials::install();
    specials::install();
    stance_normals::install();
    // misc::install();
}