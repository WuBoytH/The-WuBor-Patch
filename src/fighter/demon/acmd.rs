mod jabs;
// mod normals;
mod crouching;
mod smashes;
mod throws;
mod aerials;
mod specials;
mod escape;
mod misc;

pub fn install() {
    jabs::install();
    // normals::install();
    crouching::install();
    smashes::install();
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
    misc::install();
}