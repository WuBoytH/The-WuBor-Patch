mod normals;
mod specials;
mod escape;
mod cliff;
mod appeal;

pub fn install() {
    normals::install();
    specials::install();
    escape::install();
    appeal::install();
}