mod smashes;
mod specials;
mod escape;
mod appeal;

pub fn install() {
    smashes::install();
    specials::install();
    escape::install();
    appeal::install();
}