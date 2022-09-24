mod jabs;
mod normals;
mod smashes;
mod throws;
mod aerials;
mod specials;
mod stand;
mod step;
mod squat;
mod escape;
mod misc;

pub fn install() {
    jabs::install();
    normals::install();
    smashes::install();
    throws::install();
    aerials::install();
    specials::install();
    stand::install();
    step::install();
    squat::install();
    escape::install();
    misc::install();
}