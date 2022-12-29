mod attack_air;
mod special_n;
mod special_lw;

pub fn install() {
    attack_air::install();
    special_n::install();
    special_lw::install();
}