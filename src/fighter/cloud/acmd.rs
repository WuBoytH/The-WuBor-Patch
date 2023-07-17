mod normals;
mod aerials;
mod specials;
mod throws;
mod escape;
mod appeal;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
    throws::install();
    escape::install();
    appeal::install();
}