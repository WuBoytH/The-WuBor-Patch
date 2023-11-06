use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn yoshi_guard_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe extern "C" fn yoshi_guard_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStop_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_GUARD, yoshi_guard_exec);
    agent.status(smashline::ExecStop, *FIGHTER_STATUS_KIND_GUARD, yoshi_guard_exec_stop);
}