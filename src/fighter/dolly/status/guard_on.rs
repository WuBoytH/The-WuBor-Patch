use crate::imports::*;

unsafe extern "C" fn dolly_guard_on_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_GUARD_ON, dolly_guard_on_main);
}