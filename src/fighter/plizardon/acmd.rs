mod normals;
mod smash_attacks;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    smash_attacks::install();
    specials::install();
    escape::install();
}