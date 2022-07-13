mod normals;
mod specials;
mod escape;
mod misc;

pub fn install() {
    normals::install();
    specials::install();
    escape::install();
    misc::install();
}