use super::*;

extern "C" {
    #[link_name = "belmont_special_n_main_inner"]
    pub fn belmont_special_n_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "belmont_special_n_end_inner"]
    pub fn belmont_special_n_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_main_inner);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, belmont_special_n_end_inner);
}