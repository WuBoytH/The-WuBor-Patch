use super::*;

mod attack_air_lw;
mod special_hi_gliding;

pub mod helper;

pub fn install(agent: &mut Agent) {
    attack_air_lw::install(agent);
    special_hi_gliding::install(agent);
}