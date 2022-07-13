mod ground;
mod smashes;
mod aerials;
mod specials;
mod escape;

pub fn install() {
    ground::install();
    smashes::install();
    aerials::install();
    specials::install();
    escape::install();
}