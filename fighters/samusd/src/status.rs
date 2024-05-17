use super::*;

mod wait;

mod attack_air;

mod special_n_hold;

mod special_air_lw;

pub fn install(agent: &mut smashline::Agent) {
    wait::install(agent);

    attack_air::install(agent);

    special_n_hold::install(agent);

    special_air_lw::install(agent);
}