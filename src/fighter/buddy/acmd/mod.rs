mod ground_normals;
mod aerials;

pub fn install() {
    ground_normals::install();
    aerials::install();
}