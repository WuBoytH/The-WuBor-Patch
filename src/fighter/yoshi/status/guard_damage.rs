use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn yoshi_guard_damage_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn yoshi_guard_damage_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    yoshi_guard_exec_helper(fighter);
    fighter.sub_ftStatusUniqProcessGuardOn_execStop_Inner(L2CValue::Void());
    0.into()
}

pub fn install() {
    install_status_scripts!(
        yoshi_guard_damage_exec,
        yoshi_guard_damage_exec_stop
    );
}