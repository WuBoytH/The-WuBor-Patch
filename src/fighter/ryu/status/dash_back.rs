use {
    crate::imports::*,
    crate::fighter::common::status::movement::dash::*
};

unsafe extern "C" fn ryu_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

unsafe extern "C" fn ryu_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, ryu_dashback_pre);
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, ryu_dashback_main);
}