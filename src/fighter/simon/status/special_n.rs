use crate::imports::*;
use crate::fighter::belmont::status::special_n::*;

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_main_inner);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_end_inner);
}