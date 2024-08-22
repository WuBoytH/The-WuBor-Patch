use super::*;

// mod wait;

// mod walk;

mod dash;

// mod squat_wait;

mod landing;

mod attack;

mod attack_lw3;

mod attack_stand_2;

mod attack_step_2s;

mod landing_attack_air;

mod down;

pub fn install(agent: &mut Agent) {
    // wait::install(agent);

    // walk::install(agent);

    dash::install(agent);

    // squat_wait::install(agent);

    landing::install(agent);

    attack::install(agent);

    attack_lw3::install(agent);

    attack_stand_2::install(agent);

    attack_step_2s::install(agent);

    landing_attack_air::install(agent);

    down::install(agent);
}