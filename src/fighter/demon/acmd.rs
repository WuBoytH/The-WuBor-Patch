mod jabs;
// mod normals;
mod crouching;
mod smashes;
mod throws;
mod aerials;
mod specials;
mod stand;
mod step;
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
    stand::install();
    step::install();
    escape::install();
    misc::install();
}