use crate::imports::status_imports::*;
use super::helper::*;

#[status("yoshi", FIGHTER_STATUS_KIND_GUARD)]
unsafe fn yoshi_guard_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

#[status("yoshi", FIGHTER_STATUS_KIND_GUARD)]
unsafe fn yoshi_guard_exec_stop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuard_execStop_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

pub fn install() {
    yoshi_guard_exec::install();
    yoshi_guard_exec_stop::install();
}