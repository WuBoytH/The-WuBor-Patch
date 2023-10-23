use crate::imports::status_imports::*;
use super::super::helper::*;

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attacks4_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackXX4Start();
    mario_remove_hammer(fighter);
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attacks4_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackS4Hold();
    mario_remove_hammer(fighter);
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attacks4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        mario_attacks4_start_end,

        mario_attacks4_hold_end,

        mario_attacks4_end
    );
}