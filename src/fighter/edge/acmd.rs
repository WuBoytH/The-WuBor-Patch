mod aerials;
mod throws;
mod specials;
mod escape;
mod appeal;

pub fn install() {
    aerials::install();
    throws::install();
    specials::install();
    escape::install();
    appeal::install();
}