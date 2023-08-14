use crate::imports::status_imports::*;
use super::helper::*;

#[status("yoshi", FIGHTER_STATUS_KIND_GUARD_ON)]
unsafe fn yoshi_guard_on_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardOn_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

#[status("yoshi", FIGHTER_STATUS_KIND_GUARD_ON)]
unsafe fn yoshi_guard_on_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    yoshi_guard_exec_helper(fighter);
    fighter.sub_ftStatusUniqProcessGuardOn_execStop_Inner(L2CValue::Void());
    0.into()
}

pub fn install() {
    yoshi_guard_on_exec::install();
    yoshi_guard_on_exec_stop::install();
}