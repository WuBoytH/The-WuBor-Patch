mod aerials;
mod escape;
mod normals;

pub fn install() {
    aerials::install();
    escape::install();
    normals::install();
}
