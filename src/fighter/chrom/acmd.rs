mod movement;
mod ground_normals;
mod smash_attacks;
mod aerials;
mod specials;
mod escape;

pub fn install() {
    movement::install();
    ground_normals::install();
    smash_attacks::install();
    aerials::install();
    specials::install();
    escape::install();
}