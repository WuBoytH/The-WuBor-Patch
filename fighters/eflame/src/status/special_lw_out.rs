use super::*;
use crate::fighter::vars::element::status::special_lw_out::*;

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_LW_OUT, element_special_lw_out_main);
}