mod ground;
mod throws;
mod aerials;
mod specials;

pub fn install() {
    ground::install();
    throws::install();
    aerials::install();
    specials::install();
}