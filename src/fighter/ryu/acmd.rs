mod normals;
mod smashes;
mod specials;
mod escape;
mod appeal;

pub fn install() {
    normals::install();
    smashes::install();
    specials::install();
    escape::install();
    appeal::install();
}