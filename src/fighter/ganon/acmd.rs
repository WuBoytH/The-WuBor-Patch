mod normals;
mod smash_attacks;
mod aerials;
mod specials;

pub fn install() {
    normals::install();
    smash_attacks::install();
    aerials::install();
    specials::install();
}