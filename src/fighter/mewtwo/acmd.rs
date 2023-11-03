mod aerials;
mod escape;
mod normals;
mod smashes;

pub fn install() {
    aerials::install();
    escape::install();
    normals::install();
    smashes::install();
}
