mod jabs;
mod throws;
mod specials;

pub fn install() {
    jabs::install();
    throws::install();
    specials::install();
}