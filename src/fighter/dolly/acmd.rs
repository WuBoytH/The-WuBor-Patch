mod ground_normals;
mod smashes;
mod throws;
mod aerials;
mod specials;
mod escape;
mod misc;

pub fn install() {
    ground_normals::install();
    smashes::install();
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
    misc::install();
}