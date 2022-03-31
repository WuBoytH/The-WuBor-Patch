mod ground;
mod aerials;
mod specials;
mod misc;

pub fn install() {
    ground::install();
    aerials::install();
    specials::install();
    misc::install();
}