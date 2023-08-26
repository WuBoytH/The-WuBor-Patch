mod normals;
mod smashes;
mod specials;
mod escape;
mod misc;

pub fn install() {
    normals::install();
    smashes::install();
    specials::install();
    escape::install();
    misc::install();
}