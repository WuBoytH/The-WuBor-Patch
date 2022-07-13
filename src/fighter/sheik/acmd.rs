mod normals;
mod aerials;
mod escape;

pub fn install() {
    normals::install();
    aerials::install();
    escape::install();
}