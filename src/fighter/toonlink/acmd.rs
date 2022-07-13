mod normals;
mod aerials;
mod throws;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    aerials::install();
    throws::install();
    specials::install();
    escape::install();
}
