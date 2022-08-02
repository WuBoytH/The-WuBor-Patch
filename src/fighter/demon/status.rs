mod dash;
mod attack;
mod attacklw3;
mod attackstand2;

pub fn install() {
    dash::install();
    attack::install();
    attacklw3::install();
    attackstand2::install();
}