use super::*;

mod dash;
mod attack;

mod attack_lw3;

mod attack_stand_2;

mod landing_attack_air;
mod down;

pub fn install(agent: &mut Agent) {
    dash::install(agent);
    attack::install(agent);

    attack_lw3::install(agent);

    attack_stand_2::install(agent);

    landing_attack_air::install(agent);
    down::install(agent);
}