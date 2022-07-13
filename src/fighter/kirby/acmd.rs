mod normals;
mod aerials;
mod specials;
mod escape;
mod misc;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
    escape::install();
    misc::install();
}