mod normals;
mod smashes;
mod specials;
mod escape;
mod aerials;

pub fn install() {
    normals::install();
    smashes::install();
    specials::install();
    escape::install();
    aerials::install();
}