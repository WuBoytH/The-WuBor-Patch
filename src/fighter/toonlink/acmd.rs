mod normals;
mod aerials;
mod throws;
mod specials;
mod lasso;
mod escape;

pub fn install() {
    normals::install();
    aerials::install();
    throws::install();
    specials::install();
    lasso::install();
    escape::install();
}
