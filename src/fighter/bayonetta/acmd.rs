mod ground_normals;
mod aerials;
mod specials;
mod escape;

pub fn install() {
    ground_normals::install();
    aerials::install();
    specials::install();
    escape::install();
}