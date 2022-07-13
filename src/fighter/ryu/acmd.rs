mod normals;
mod specials;
mod escape;
mod appeal;

pub fn install() {
    normals::install();
    specials::install();
    escape::install();
    appeal::install();
}