use super::*;

mod attack;
mod attack_air;
mod special_air_s_d;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);
    attack_air::install(agent);
    special_air_s_d::install(agent);
}
