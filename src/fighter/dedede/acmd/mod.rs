mod ground_normals;
mod smash_attacks;
mod specials;

pub fn install() {
    ground_normals::install();
    smash_attacks::install();
    specials::install();
}