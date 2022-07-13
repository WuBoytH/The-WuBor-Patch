mod ground_normals;
mod smash_attacks;
mod aerials;
mod specials;

pub fn install() {
    ground_normals::install();
    smash_attacks::install();
    aerials::install();
    specials::install();
}