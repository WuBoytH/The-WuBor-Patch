pub mod dash;
mod jump_squat;
mod tread_jump;
mod shield;
mod escape;
pub mod attack;
// mod attack_3_common;
mod attack_air;
mod catch;
mod damage_air;
mod damage_fall;
mod sub_transitions;

pub fn install() {
    dash::install();
    jump_squat::install();
    tread_jump::install();
    shield::install();
    escape::install();
    attack::install();
    // attack_3_common::install();
    attack_air::install();
    catch::install();
    damage_air::install();
    damage_fall::install();
    sub_transitions::install();
}