mod ground;
mod aerials;
mod specials;
mod escape;
mod misc;

pub fn install() {
    ground::install();
    aerials::install();
    specials::install();
    escape::install();
    misc::install();
}