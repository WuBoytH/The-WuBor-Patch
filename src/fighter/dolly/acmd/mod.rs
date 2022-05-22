mod ground_normals;
mod smashes;
mod throws;
mod specials;
mod misc;

pub fn install() {
    ground_normals::install();
    smashes::install();
    throws::install();
    specials::install();
    misc::install();
}