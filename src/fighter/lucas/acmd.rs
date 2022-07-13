mod normals;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod escape;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    specials::install();
    throws::install();
    escape::install();
}
