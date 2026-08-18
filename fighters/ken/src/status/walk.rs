use super::*;

extern "C" {
    #[link_name = "fgc_walk_back_main"]
    pub fn fgc_walk_back_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_WALK_BACK, fgc_walk_back_main);
}
