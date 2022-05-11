mod aerials;
mod specials;
mod misc;

pub fn install() {
    aerials::install();
    specials::install();
    misc::install();
}