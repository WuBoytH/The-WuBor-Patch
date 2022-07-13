mod ground_normals;
mod aerials;
mod escape;

pub fn install() {
    ground_normals::install();
    aerials::install();
    escape::install();
}