mod normals;
mod smash_attacks;
mod throws;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    smash_attacks::install();
    throws::install();
    specials::install();
    escape::install();
}