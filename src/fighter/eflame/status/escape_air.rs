use crate::imports::*;

unsafe extern "C" fn eflame_escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_EscapeAir()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR, eflame_escape_air_main);
}