mod normals;
mod smashes;
mod aerials;
mod throws;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    throws::install();
    specials::install();
    escape::install();
}
