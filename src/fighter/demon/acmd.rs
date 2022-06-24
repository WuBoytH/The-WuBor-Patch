mod jabs;
// mod tilts;
mod throws;
mod specials;
mod misc;

pub fn install() {
    jabs::install();
    // tilts::install();
    throws::install();
    specials::install();
    misc::install();
}