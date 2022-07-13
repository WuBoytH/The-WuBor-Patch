mod throws;
mod aerials;
mod specials;
mod escape;

pub fn install() {
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
}