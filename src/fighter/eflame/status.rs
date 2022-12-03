mod attack;
mod attack_air;
mod escape_air;

pub fn install() {
    attack::install();
    attack_air::install();
    escape_air::install();
}