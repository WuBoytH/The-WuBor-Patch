mod normals;
mod smashes;
mod aerials;
mod specials;
mod lasso;
mod escape;

pub fn install() {
    normals::install();
    smashes::install();
    aerials::install();
    specials::install();
    lasso::install();
    escape::install();
}