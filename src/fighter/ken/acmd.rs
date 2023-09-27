mod normals;
mod smashes;
mod specials;
mod escape;

pub fn install() {
    normals::install();
    smashes::install();
    specials::install();
    escape::install();
}