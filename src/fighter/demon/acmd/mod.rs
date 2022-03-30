mod jabs;
mod throws;
mod specials;
mod misc;

pub fn install() {
    jabs::install();
    throws::install();
    specials::install();
    misc::install();
}