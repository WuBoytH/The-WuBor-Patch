mod ground;
mod throws;
mod aerials;
mod specials;
mod misc;

pub fn install() {
    ground::install();
    throws::install();
    aerials::install();
    specials::install();
    misc::install();
}