use crate::imports::status_imports::*;

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn dolly_guard_off_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_GUARD_OFF, dolly_guard_off_main);
}