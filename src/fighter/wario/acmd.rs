mod throws;
mod specials;
mod escape;

pub fn install() {
    throws::install();
    specials::install();
    escape::install();
}
