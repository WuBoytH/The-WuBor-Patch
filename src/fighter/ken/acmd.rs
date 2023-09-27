mod normals;
mod smashes;
mod escape;

pub fn install() {
    normals::install();
    smashes::install();
    escape::install();
}