mod normals;
mod specials;
mod lasso;
mod escape;

pub fn install() {
    normals::install();
    specials::install();
    lasso::install();
    escape::install();
}