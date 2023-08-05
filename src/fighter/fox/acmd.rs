mod aerials;
mod normals;
mod specials;
mod escape;

pub fn install() {
    aerials::install();
    normals::install();
    specials::install();
    escape::install();
}