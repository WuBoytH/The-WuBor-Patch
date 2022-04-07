mod ground;
mod aerials;
mod specials;

pub fn install() {
    ground::install();
    aerials::install();
    specials::install();
}