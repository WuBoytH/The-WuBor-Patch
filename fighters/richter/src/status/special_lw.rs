use super::*;

extern "C" {
    #[link_name = "belmont_special_lw_pre_inner"]
    pub fn belmont_special_lw_pre_inner(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "belmont_special_lw_main_inner"]
    pub fn belmont_special_lw_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "belmont_special_lw_end_inner"]
    pub fn belmont_special_lw_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_pre_inner);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_main_inner);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, belmont_special_lw_end_inner);
}