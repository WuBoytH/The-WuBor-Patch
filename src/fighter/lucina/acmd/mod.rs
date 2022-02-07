mod ground_normals;
mod smash_attacks;
mod aerials;
mod specials;
mod grabs;
mod misc;

pub fn install() {
    ground_normals::install();
    smash_attacks::install();
    aerials::install();
    specials::install();
    grabs::install();
    misc::install();
}