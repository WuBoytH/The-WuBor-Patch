mod smashes;
mod aerials;
mod escape;

pub fn install() {
    smashes::install();
    aerials::install();
    escape::install();
}
