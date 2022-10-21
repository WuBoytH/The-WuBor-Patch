mod normals;
mod smash_attacks;
mod escape;

pub fn install() {
    normals::install();
    smash_attacks::install();
    escape::install();
}