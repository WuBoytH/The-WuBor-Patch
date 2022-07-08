mod normals;
mod specials;
mod misc;

pub fn install() {
    normals::install();
    specials::install();
    misc::install();
}