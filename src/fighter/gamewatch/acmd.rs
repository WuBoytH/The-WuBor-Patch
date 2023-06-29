mod aerials;
mod specials;
mod escape;

pub fn install() {
    aerials::install();
    specials::install();
    escape::install();
}