use crate::imports::status_imports::*;

#[status("lucario", FIGHTER_STATUS_KIND_LANDING)]
unsafe fn lucario_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    fighter.status_pre_Landing()
}

#[status("lucario", FIGHTER_STATUS_KIND_LANDING_LIGHT)]
unsafe fn lucario_landing_light_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    fighter.status_pre_LandingLight()
}

#[status("lucario", FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)]
unsafe fn lucario_landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    fighter.status_pre_LandingAttackAir()
}

pub fn install() {
    lucario_landing_pre::install();
    lucario_landing_light_pre::install();
    lucario_landing_attack_air_pre::install();
}