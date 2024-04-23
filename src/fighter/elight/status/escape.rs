use crate::imports::*;

unsafe extern "C" fn elight_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE, elight_escape_main);
}