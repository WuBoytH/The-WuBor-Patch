mod ground;
mod throws;
mod aerials;
mod specials;
mod escape;
// mod misc;

pub fn install() {
    ground::install();
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
    // misc::install();
}