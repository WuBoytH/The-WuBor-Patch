use super::*;
use crate::fighter::belmont::status::special_lw::*;

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_pre_inner);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_main_inner);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_end_inner);
}