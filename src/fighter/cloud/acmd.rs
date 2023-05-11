mod normals;
mod specials;
mod throws;
mod escape;

pub fn install() {
    normals::install();
    specials::install();
    throws::install();
    escape::install();
}