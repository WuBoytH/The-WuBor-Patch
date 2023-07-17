mod normals;
mod aerials;
mod specials;
mod throws;
mod escape;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
    throws::install();
    escape::install();
}
