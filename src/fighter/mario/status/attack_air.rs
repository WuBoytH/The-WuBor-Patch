use crate::imports::status_imports::*;
use super::super::helper::*;

#[status("mario", FIGHTER_STATUS_KIND_ATTACK_AIR)]
unsafe fn mario_attackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

#[status("mario", FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)]
unsafe fn mario_landingattackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_LandingAttackAir();
    mario_remove_hammer(fighter);
    0.into()
}

pub fn install() {
    mario_attackair_end::install();
    mario_landingattackair_end::install();
}