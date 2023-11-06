use crate::imports::status_imports::*;
use crate::fighter::belmont::status::special_lw::*;

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_pre_inner);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_main_inner);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_end_inner);
}