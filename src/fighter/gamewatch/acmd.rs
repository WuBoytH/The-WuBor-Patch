mod aerials;
mod specials;
mod escape;
mod normals;

pub fn install() {
    aerials::install();
    specials::install();
    escape::install();
    normals::install();
}