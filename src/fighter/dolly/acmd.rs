mod normals;
mod smashes;
mod catch;
mod throws;
mod aerials;
mod specials;
mod escape;
mod misc;

pub fn install() {
    normals::install();
    smashes::install();
    catch::install();
    throws::install();
    aerials::install();
    specials::install();
    escape::install();
    misc::install();
}