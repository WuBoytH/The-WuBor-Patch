use crate::imports::status_imports::*;

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samusd_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Wait()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_WAIT, samusd_wait_main);
}