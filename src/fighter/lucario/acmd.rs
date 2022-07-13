mod normals;
mod throws;
mod aerials;
mod specials;
mod escape;
// mod misc;

pub fn install() {
    normals::install();
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
    // misc::install();
}