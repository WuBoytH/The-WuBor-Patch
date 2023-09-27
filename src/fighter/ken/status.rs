mod dash_back;
mod attack;
mod attack_lw4_start;
mod attack_lw4;

pub fn install() {
    dash_back::install();
    attack::install();
    attack_lw4_start::install();
    attack_lw4::install();
}