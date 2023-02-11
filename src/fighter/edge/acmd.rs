mod aerials;
mod throws;
mod specials;
mod escape;
mod misc;

pub fn install() {
    aerials::install();
    throws::install();
    specials::install();
    escape::install();
    misc::install();
}