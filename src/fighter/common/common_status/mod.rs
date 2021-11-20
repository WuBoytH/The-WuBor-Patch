pub mod dash;
mod jump_squat;
mod tread_jump;
mod shield;
pub mod attack;
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
    attack::install();
    attack_air::install();
    catch::install();
    damage_air::install();
    damage_fall::install();
    sub_transitions::install();
}