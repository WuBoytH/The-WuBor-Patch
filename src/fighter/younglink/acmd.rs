mod specials;
mod lasso;
mod escape;

pub fn install() {
    specials::install();
    lasso::install();
    escape::install();
}
