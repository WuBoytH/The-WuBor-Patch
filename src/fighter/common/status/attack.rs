pub mod attack;
mod attack_100;
mod attack_dash;
mod attack_3;
// mod attack_3_common;
mod attack_4;
mod attack_air;

pub fn install() {
    attack::install();
    attack_100::install();
    attack_dash::install();
    attack_3::install();
    attack_4::install();
    attack_air::install();
}