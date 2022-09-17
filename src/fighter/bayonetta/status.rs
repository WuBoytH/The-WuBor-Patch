mod attack;
mod attack_air;
mod special_air_s_d;

pub fn install() {
    attack::install();
    attack_air::install();
    special_air_s_d::install();
}
