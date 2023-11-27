mod normals;
mod smashes;
mod aerials;
mod specials;
mod escape;
mod cliff;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    specials::install();
    escape::install();
}