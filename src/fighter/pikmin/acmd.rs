mod normals;
mod catch;
mod escape;

pub fn install() {
    normals::install();
    catch::install();
    escape::install();
}