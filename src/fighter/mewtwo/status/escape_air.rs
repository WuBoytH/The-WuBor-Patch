use crate::imports::*;

unsafe extern "C" fn mewtwo_escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_EscapeAir()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR, mewtwo_escape_air_main);
}