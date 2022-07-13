mod normals;
mod smashes;
mod aerials;
mod escape;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    escape::install();
}