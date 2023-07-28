mod normals;
mod aerials;
mod specials;
mod catch;
mod escape;
mod smashes;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
    catch::install();
    escape::install();
    smashes::install();
}