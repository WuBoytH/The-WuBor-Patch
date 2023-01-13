mod attack_air;
mod special_lw;
mod special_lw_out;
mod escape_air;

pub fn install() {
    attack_air::install();
    special_lw::install();
    special_lw_out::install();
    escape_air::install();
}