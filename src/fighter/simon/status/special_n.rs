use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_n::*;

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_main_inner);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_end_inner);
}