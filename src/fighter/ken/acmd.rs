mod normals;
mod specials;
mod escape;
mod cliff;
mod misc;

pub fn install() {
    normals::install();
    specials::install();
    escape::install();
    cliff::install();
    misc::install();
}