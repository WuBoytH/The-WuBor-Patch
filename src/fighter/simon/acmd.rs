mod normals;
mod smashes;
mod aerials;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    specials::install();
    escape::install();
}
