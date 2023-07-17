mod attack_air_lw;
mod special_hi_gliding;

pub mod helper;

pub fn install() {
    attack_air_lw::install();
    special_hi_gliding::install();
}