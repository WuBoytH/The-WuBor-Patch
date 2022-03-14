mod ground_normals;
mod smash_attacks;
mod throws;
mod specials;

pub fn install() {
    ground_normals::install();
    smash_attacks::install();
    throws::install();
    specials::install();
}