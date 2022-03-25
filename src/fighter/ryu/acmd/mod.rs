mod ground;
mod specials;
mod misc;

pub fn install() {
    ground::install();
    specials::install();
    misc::install();
}