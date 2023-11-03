mod normals;
mod aerials;
mod specials;
mod catch;
mod throw;
mod escape;

pub fn install() {
    normals::install();
    aerials::install();
    specials::install();
    catch::install();
    throw::install();
    escape::install();
}
