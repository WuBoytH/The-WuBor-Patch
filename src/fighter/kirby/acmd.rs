mod normals;
mod aerials;
mod specials;
mod copy;
mod escape;
mod misc;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
    copy::install();
    escape::install();
    misc::install();
}