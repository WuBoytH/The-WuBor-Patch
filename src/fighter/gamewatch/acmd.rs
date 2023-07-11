mod aerials;
mod specials;
mod escape;
mod normals;
mod smashes;

pub fn install() {
    aerials::install();
    specials::install();
    escape::install();
    normals::install();
    smashes::install();
}