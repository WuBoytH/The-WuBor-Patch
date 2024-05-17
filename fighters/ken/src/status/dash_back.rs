use super::*;

extern "C" {
    #[link_name = "fgc_dashback_pre"]
    pub fn fgc_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "fgc_dashback_main"]
    pub fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, fgc_dashback_pre);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, fgc_dashback_main);
}