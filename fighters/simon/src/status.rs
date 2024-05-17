use super::*;

mod attack_air;
mod special_n;
mod special_lw;

mod helper;

pub fn install(agent: &mut smashline::Agent) {
    attack_air::install(agent);
    special_n::install(agent);
    special_lw::install(agent);
}