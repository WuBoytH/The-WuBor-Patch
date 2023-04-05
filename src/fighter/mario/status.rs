mod attack_s4;
mod attack_air;
mod special_s;
mod special_lw;
mod rebirth;

pub fn install() {
    attack_s4::install();
    attack_air::install();
    special_s::install();
    special_lw::install();
    rebirth::install();
}
