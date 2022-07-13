mod normals;
mod aerials;
mod throws;
mod escape;

pub fn install() {
    normals::install();
    aerials::install();
    throws::install();
    escape::install();
}
