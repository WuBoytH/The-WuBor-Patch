use crate::imports::status_imports::*;
use super::helper::*;

unsafe extern "C" fn yoshi_guard_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

unsafe extern "C" fn yoshi_guard_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStop_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_GUARD, yoshi_guard_exec);
    agent.status(smashline::ExecStop, *FIGHTER_STATUS_KIND_GUARD, yoshi_guard_exec_stop);
}