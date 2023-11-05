use crate::imports::status_imports::*;
use crate::fighter::element::status::special_lw_out::*;

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT, element_special_lw_out_main);
}