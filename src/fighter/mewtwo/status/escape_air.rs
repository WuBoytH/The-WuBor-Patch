use crate::imports::status_imports::*;

#[status_script( agent = "mewtwo", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN )]
unsafe extern "C" fn mewtwo_escape_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_EscapeAir()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR, mewtwo_escape_air_main);
}