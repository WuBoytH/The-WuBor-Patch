use super::*;

mod attack;
mod attack_lw3;
mod attack_air;
mod special_n;
mod special_hi;
mod special_lw;

pub fn install(agent: &mut Agent) {
    attack::install(agent);
    attack_lw3::install(agent);
    attack_air::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
}