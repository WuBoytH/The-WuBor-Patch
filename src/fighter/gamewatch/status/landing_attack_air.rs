use crate::imports::status_imports::*;

#[status("gamewatch", FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)]
unsafe fn gamewatch_landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_LandingAttackAir()
}

pub fn install() {
    gamewatch_landing_attack_air_pre::install();
}