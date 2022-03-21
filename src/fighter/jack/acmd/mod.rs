mod throws;
mod aerials;
mod specials;

pub fn install() {
    throws::install();
    aerials::install();
    specials::install();
}