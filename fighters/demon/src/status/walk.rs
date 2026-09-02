use super::*;

extern "C" {
    #[link_name = "fgc_walk_back_main"]
    pub fn fgc_walk_back_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn demon_walk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Walk()
}

unsafe extern "C" fn demon_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Walk()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_WALK, demon_walk_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_WALK, demon_walk_main);

    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_WALK_BACK, fgc_walk_back_main);
}
