mod ground;
mod aerials;
mod specials;
mod escape;

pub fn install() {
    ground::install();
    aerials::install();
    specials::install();
    escape::install();
}