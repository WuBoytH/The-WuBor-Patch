mod ground;
mod smashes;
mod throws;
mod aerials;
mod specials;

pub fn install() {
    ground::install();
    smashes::install();
    throws::install();
    aerials::install();
    specials::install();
}