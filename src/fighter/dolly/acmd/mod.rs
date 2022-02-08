mod ground_normals;
mod throws;
mod specials;

pub fn install() {
    ground_normals::install();
    throws::install();
    specials::install();
}