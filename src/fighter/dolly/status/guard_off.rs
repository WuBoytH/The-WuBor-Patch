use crate::imports::status_imports::*;

unsafe extern "C" fn dolly_guard_off_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_GUARD_OFF, dolly_guard_off_main);
}