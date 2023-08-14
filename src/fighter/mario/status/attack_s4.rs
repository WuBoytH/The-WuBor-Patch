use crate::imports::status_imports::*;
use super::super::helper::*;

#[status("mario", FIGHTER_STATUS_KIND_ATTACK_S4_START)]
unsafe fn mario_attacks4_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackXX4Start();
    mario_remove_hammer(fighter);
    0.into()
}

#[status("mario", FIGHTER_STATUS_KIND_ATTACK_S4_HOLD)]
unsafe fn mario_attacks4_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackS4Hold();
    mario_remove_hammer(fighter);
    0.into()
}

#[status("mario", FIGHTER_STATUS_KIND_ATTACK_S4)]
unsafe fn mario_attacks4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

pub fn install() {
    mario_attacks4_start_end::install();
    mario_attacks4_hold_end::install();
    mario_attacks4_end::install();
}