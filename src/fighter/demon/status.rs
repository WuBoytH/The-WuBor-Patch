mod dash;
mod attack;
mod attacklw3;
mod attackstand2;
mod landing_attack_air;

pub fn install() {
    dash::install();
    attack::install();
    attacklw3::install();
    attackstand2::install();
    landing_attack_air::install();
}