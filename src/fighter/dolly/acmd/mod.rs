mod ground_normals;
mod throws;
mod specials;
mod misc;

pub fn install() {
    ground_normals::install();
    throws::install();
    specials::install();
    misc::install();
}