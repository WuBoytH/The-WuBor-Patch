use super::*;

extern "C" {
    #[link_name = "element_special_lw_end"]
    pub fn element_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, element_special_lw_end);
}