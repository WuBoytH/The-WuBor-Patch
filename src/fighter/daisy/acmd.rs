mod normals;
mod aerials;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
    escape::install();
}