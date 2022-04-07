mod ground;
mod smashes;
mod aerials;
mod specials;

pub fn install() {
    ground::install();
    smashes::install();
    aerials::install();
    specials::install();
}