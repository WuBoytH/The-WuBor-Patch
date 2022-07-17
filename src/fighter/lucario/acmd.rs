mod normals;
mod smashes;
mod throws;
mod aerials;
mod specials;
mod escape;
// mod misc;

pub fn install() {
    normals::install();
    smashes::install();
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
    // misc::install();
}