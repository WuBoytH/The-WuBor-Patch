use crate::imports::status_imports::*;

unsafe extern "C" fn elight_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ESCAPE, elight_escape_main);
}