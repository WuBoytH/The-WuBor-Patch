mod specials;
mod throws;
mod escape;

pub fn install() {
    specials::install();
    throws::install();
    escape::install();
}