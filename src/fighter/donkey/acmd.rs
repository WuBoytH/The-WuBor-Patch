mod normals;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    specials::install();
    escape::install();
}