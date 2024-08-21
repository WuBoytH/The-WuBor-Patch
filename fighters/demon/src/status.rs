use super::*;

// mod wait;

// mod walk;

mod dash;

// mod squat_wait;

mod landing;

mod attack;

mod attacklw3;

mod attackstand2;

mod landing_attack_air;

mod down;

pub fn install(agent: &mut Agent) {
    // wait::install(agent);

    // walk::install(agent);

    dash::install(agent);

    // squat_wait::install(agent);

    landing::install(agent);

    attack::install(agent);

    attacklw3::install(agent);

    attackstand2::install(agent);

    landing_attack_air::install(agent);

    down::install(agent);
}