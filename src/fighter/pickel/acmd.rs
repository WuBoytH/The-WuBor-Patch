mod normals;
mod smashes;
mod aerials;
mod specials;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    specials::install();
}