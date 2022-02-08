mod ground_normals;
mod aerials;
mod specials;

pub fn install() {
    ground_normals::install();
    aerials::install();
    specials::install();
}