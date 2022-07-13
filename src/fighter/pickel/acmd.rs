mod normals;
mod aerials;
mod specials;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
}