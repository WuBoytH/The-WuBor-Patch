use crate::imports::*;
use super::helper::*;

unsafe extern "C" fn yoshi_guard_damage_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

unsafe extern "C" fn yoshi_guard_damage_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    yoshi_guard_exec_helper(fighter);
    fighter.sub_ftStatusUniqProcessGuardOn_execStop_Inner(L2CValue::Void());
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, yoshi_guard_damage_exec);
    agent.status(smashline::ExecStop, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, yoshi_guard_damage_exec_stop);
}