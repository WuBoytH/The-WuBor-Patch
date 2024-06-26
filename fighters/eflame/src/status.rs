use super::*;

mod attack_air;
mod special_lw;
mod special_lw_out;
mod escape_air;

mod change_helper;

pub fn install(agent: &mut Agent) {
    attack_air::install(agent);
    special_lw::install(agent);
    special_lw_out::install(agent);
    escape_air::install(agent);
}