mod jabs;
// mod tilts;
mod throws;
mod specials;
mod escape;
mod misc;

pub fn install() {
    jabs::install();
    // tilts::install();
    throws::install();
    specials::install();
    escape::install();
    misc::install();
}